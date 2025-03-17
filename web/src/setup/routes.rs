use axum::{Router, middleware, routing};
use tower_http::compression::CompressionLayer;

use crate::{
    middlewares::{add_corbado_project_id, no_cache, require_admin, require_auth},
    routes,
    state::AppState,
};

pub(super) fn setup(app: Router<AppState>, state: AppState) -> Router<AppState> {
    let health_router = Router::new()
        .route("/", routing::get(routes::health::health))
        .layer(middleware::from_fn(no_cache));

    let static_router = Router::new()
        .route("/{*path}", routing::get(routes::serve_static::static_path))
        .layer(CompressionLayer::new());

    let auth_router = Router::new()
        .route("/login/", routing::get(routes::login::login))
        .route("/callback/", routing::get(routes::login::after_login));

    let admin_router = Router::new()
        .route(
            "/exercise/schemas/",
            routing::get(routes::admin::exercise_schema_list),
        )
        .route(
            "/exercise/schemas/new/",
            routing::get(routes::admin::exercise_schema_new)
                .post(routes::admin::exercise_schema_post),
        )
        .route(
            "/exercise/schemas/{id}/",
            routing::get(routes::admin::exercise_schema_edit)
                .post(routes::admin::exercise_schema_post),
        )
        .route(
            "/exercise/schemas/{id}/json/",
            routing::get(routes::admin::exercise_schema_json),
        )
        .route(
            "/exercise/{id}/",
            routing::get(routes::admin::exercise_edit).post(routes::admin::exercise_post),
        )
        .route(
            "/exercise/new/",
            routing::get(routes::admin::exercise_new).post(routes::admin::exercise_post),
        )
        .layer(middleware::from_fn_with_state(state.clone(), require_admin));

    let exercise_router = Router::new()
        .route("/{id}/", routing::get(routes::exercise_run::run))
        .route(
            "/{id}/submit/",
            routing::post(routes::exercise_run::submit_solution),
        )
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    let main_page_router = Router::new()
        .route("/", routing::get(routes::main::main_page))
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    app.merge(main_page_router)
        .nest("/static/", static_router)
        .nest("/exercise/", exercise_router)
        .nest("/auth/", auth_router)
        .nest("/admin/", admin_router)
        .nest("/health/", health_router)
        .layer(middleware::from_fn_with_state(
            state.clone(),
            add_corbado_project_id,
        ))
}
