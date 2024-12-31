use axum::http::StatusCode;
use displaydoc::Display;
use thiserror::Error;
use tracing::error;

use crate::extractors::AuthExtractorError;

pub type Result<T, E = SqlGrimoireError> = std::result::Result<T, E>;

/// All the errors which can be encountered by Platan web interface
#[derive(Debug, Display, Error)]
pub enum SqlGrimoireError {
    // 5xx errors:
    /// Raw SQLx error: {0:?}
    SqlxError(#[from] sqlx::Error),
    /// {0:?}
    EyreError(#[from] eyre::Report),
    /// Problem with CSRF: {0:?}
    CsrfTokenError(#[from] axum_csrf::CsrfError),
    /// Error while extracting user claims: {0:?}
    AuthExtractorError(#[from] crate::extractors::AuthExtractorError),

    // 4xx errors:
    /// Not found: {0:?}
    NotFound(String),
}

impl axum::response::IntoResponse for SqlGrimoireError {
    fn into_response(self) -> axum::response::Response {
        error!(
            %self,
            "Error occurred while handling request, returning error response"
        );

        let code = match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::AuthExtractorError(AuthExtractorError::NoSessionCookie) => {
                StatusCode::UNAUTHORIZED
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (code, self.to_string()).into_response()
    }
}

impl SqlGrimoireError {
    pub fn not_found(details: impl AsRef<str>) -> Self {
        Self::NotFound(details.as_ref().to_owned())
    }
}
