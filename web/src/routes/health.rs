use axum::{Json, debug_handler};

#[derive(Debug, serde::Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

/// `/health` endpoint handler.
///
/// Returns `200 OK` status + json
///
/// ```json
/// {
///     "status": "ok",
/// }
#[debug_handler]
#[tracing::instrument(skip_all)]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}
