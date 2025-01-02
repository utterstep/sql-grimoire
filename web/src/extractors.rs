use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{self, request::Parts},
    response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::CookieJar;
use axum_jwt_auth::{Error as JwtDecoderError, JwtDecoder};
use displaydoc::Display;
use eyre::WrapErr;
use thiserror::Error;
use tracing::{error, trace};

use crate::{
    db::user,
    models::user::{User, UserClaims},
    state::AppState,
};

#[derive(Debug, Error, Display)]
/// Errors which can be encountered while extracting auth data from request.
pub enum AuthExtractorError {
    /// Session cookie is not present
    NoSessionCookie,
    /// JWT Validation error: {0:?}
    JWTValidationError(JwtDecoderError),
    /// Token is expired
    TokenExpired,
    /// Token is not yet valid
    TokenNotYetValid,
    /// Token issuer is not valid
    InvalidIssuer,
    /// User not found in the database
    UserNotFound,
    /// Database error: {0:?}
    DatabaseError(#[from] sqlx::Error),
    /// {0:?}
    Other(#[from] eyre::Report),
}

impl IntoResponse for AuthExtractorError {
    fn into_response(self) -> Response {
        match self {
            AuthExtractorError::UserNotFound => {
                (http::StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
            _ => (http::StatusCode::UNAUTHORIZED, self.to_string()).into_response(),
        }
    }
}

const SESSION_COOKIE: &str = "cbo_session_token";

struct SessionToken(String);

#[async_trait]
impl FromRequestParts<AppState> for SessionToken {
    type Rejection = AuthExtractorError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            // Safe to unwrap – Result<_, Infalible> is returned
            .expect("Failed to unwrap Result<_, Infallible> while getting signed cookie jar from request");

        trace!("Got signed cookie jar from request");

        let session_token = jar
            .get(SESSION_COOKIE)
            .ok_or(AuthExtractorError::NoSessionCookie)?
            .value()
            .to_string();

        trace!("Got ID token from cookie jar");

        Ok(SessionToken(session_token))
    }
}

#[async_trait]
impl FromRequestParts<AppState> for UserClaims {
    type Rejection = AuthExtractorError;

    #[tracing::instrument(skip(parts, state))]
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let session_token = SessionToken::from_request_parts(parts, state).await?;

        let claims: UserClaims = state
            .jwks_decoder()
            .decode(&session_token.0)
            .map_err(AuthExtractorError::JWTValidationError)
            .map(|token_data| token_data.claims)?;

        let now = time::OffsetDateTime::now_utc();

        if claims.exp() < &now {
            return Err(AuthExtractorError::TokenExpired);
        }

        if claims.nbf() > &now {
            return Err(AuthExtractorError::TokenNotYetValid);
        }

        if claims.iss() != state.config().corbado_host() {
            error!("Invalid issuer: {}", claims.iss());

            return Err(AuthExtractorError::InvalidIssuer);
        }

        Ok(claims)
    }
}

#[async_trait]
impl FromRequestParts<AppState> for User {
    type Rejection = AuthExtractorError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let claims = UserClaims::from_request_parts(parts, state).await?;
        let mut conn = state.db().acquire().await?;

        let user = user::get_user(&mut conn, &claims)
            .await
            .wrap_err("Failed to query user from DB")?;

        match user {
            Some(user) => Ok(user),
            None => Err(AuthExtractorError::UserNotFound),
        }
    }
}
