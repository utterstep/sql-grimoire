use axum::{
    body::Body,
    extract::State,
    http::{self, Method, Request},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::cookie::{self, Cookie, CookieJar};

use crate::{models::user::User, state::AppState};

/// HTTP header to disable caching.
///
/// It's better to use [`crate::middlewares::no_cache`] instead of using this directly.
pub const NO_CACHE: (http::HeaderName, &str) = (
    http::header::CACHE_CONTROL,
    "private, no-store, no-cache, must-revalidate, post-check=0, pre-check=0, no-transform",
);

/// Sets the `Cache-Control: no-cache ...` header on the response.
pub async fn no_cache(method: Method, request: Request<Body>, next: Next) -> impl IntoResponse {
    let response = next.run(request).await;

    match method {
        Method::GET | Method::HEAD | Method::OPTIONS => ([NO_CACHE], response).into_response(),
        _ => response,
    }
}

/// Middleware to check if the user is admin.
pub async fn require_admin(user: User, request: Request<Body>, next: Next) -> impl IntoResponse {
    if !user.is_admin() {
        return (http::StatusCode::FORBIDDEN, "Forbidden").into_response();
    }

    next.run(request).await
}

const CORBADO_PROJECT_ID_COOKIE: &str = "corbado_project_id";

/// Middleware to add the Corbado project ID to cookies, if it's not already set.
///
/// Gets the projectId from the state.
pub async fn add_corbado_project_id(
    state: State<AppState>,
    cookies: CookieJar,
    request: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    let response = next.run(request).await;

    let project_id = state.config().corbado_project_id();

    if let Some(cookie) = cookies.get(CORBADO_PROJECT_ID_COOKIE) {
        if cookie.value() == project_id {
            return response.into_response();
        }
    }

    let cookies = {
        let mut cookie = Cookie::new(CORBADO_PROJECT_ID_COOKIE, project_id.to_string());
        cookie.set_same_site(cookie::SameSite::Lax);

        cookies.add(cookie)
    };

    (cookies, response).into_response()
}
