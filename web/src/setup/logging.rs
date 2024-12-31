use std::sync::Arc;

use axum::{http::header, Router};
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    sensitive_headers::{SetSensitiveRequestHeadersLayer, SetSensitiveResponseHeadersLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse},
};

/// Sets up logging for the application.
///
/// ___Important:___ this should be added as a last layer, so that it can log all requests,
/// even those that fail in the middle of the stack.
pub(super) fn setup<S: Clone + Send + Sync + 'static>(app: Router<S>) -> Router<S> {
    let hidden_headers: Arc<[_]> = Arc::new([header::COOKIE, header::SET_COOKIE]);

    let layers = ServiceBuilder::new()
        // 1. Set request ID
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        // 2. Set sensitive request headers, so they won't be logged
        .layer(SetSensitiveRequestHeadersLayer::from_shared(Arc::clone(
            &hidden_headers,
        )))
        // 3. Trace requests
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_response(DefaultOnResponse::new().include_headers(false)),
        )
        // 4. Set sensitive response headers, so they won't be logged
        .layer(SetSensitiveResponseHeadersLayer::from_shared(Arc::clone(
            &hidden_headers,
        )))
        // 5. Propagate request ID to response headers
        .layer(PropagateRequestIdLayer::x_request_id())
        .into_inner();

    app.layer(layers)
}
