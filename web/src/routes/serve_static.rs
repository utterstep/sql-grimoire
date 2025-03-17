use axum::{
    debug_handler,
    extract::Path,
    http::{HeaderValue, StatusCode, header},
    response::IntoResponse,
};

use crate::static_files::StaticFile;

#[debug_handler]
#[tracing::instrument]
pub async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');

    if let Some(data) = StaticFile::get(path) {
        let mut header_map = header::HeaderMap::new();
        header_map.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_str(data.mime.as_ref()).expect("Failed to create header value"),
        );
        header_map.insert(
            header::CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=604800"),
        );

        (StatusCode::OK, header_map, data.content).into_response()
    } else {
        (StatusCode::NOT_FOUND).into_response()
    }
}
