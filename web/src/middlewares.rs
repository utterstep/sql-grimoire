use axum::{
    body::Body,
    http::{self, Method, Request},
    middleware::Next,
    response::IntoResponse,
};

use crate::models::user::User;

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
