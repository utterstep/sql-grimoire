use axum::{
    body::Body,
    http::{Method, Request},
    middleware::{self, Next},
    response::IntoResponse,
    Router,
};
use axum_csrf::{CsrfConfig, CsrfLayer, CsrfToken, Key};
use axum_helmet::{
    ContentSecurityPolicy, Helmet, HelmetLayer, ReferrerPolicy, XFrameOptions, XXSSProtection,
};
use secrecy::ExposeSecret;

use crate::config::Config;

/// Sets the CSRF cookie on the response.
///
/// This middleware is only applied to `GET`, `HEAD` and `OPTIONS` requests.
async fn set_csrf_cookie(
    csrf_token: CsrfToken,
    method: Method,
    request: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    let response = next.run(request).await;

    match method {
        Method::GET | Method::HEAD | Method::OPTIONS => (csrf_token, response).into_response(),
        _ => response,
    }
}

pub(super) fn setup<S: Clone + Send + Sync + 'static>(
    app: Router<S>,
    config: &Config,
) -> Router<S> {
    // let cookie_domain = config.domain_only().to_owned().into();

    let csrf_config = CsrfConfig::default()
        // .with_cookie_domain(Some(cookie_domain))
        .with_secure(true)
        .with_cookie_name("_csrf")
        .with_key(Some(Key::from(
            config.secret_key().expose_secret().as_bytes(),
        )));

    let csp = ContentSecurityPolicy::new()
        .connect_src(vec![
            "'self'",
            // FIXME: dynamically set this to the app host
            "*.utterstep.app",
            "*.frontendapi.corbado.io",
            // goatcounter
            "esm.sh",
            "cdn.jsdelivr.net",
        ])
        .default_src(vec!["'self'"])
        .script_src(vec![
            "'self'",
            "cdn.jsdelivr.net",
            "esm.sh",
            "unpkg.com",
            "'unsafe-eval'",
        ])
        // FIXME: remove unsafe-inline
        .style_src(vec![
            "'self'",
            "unpkg.com",
            "fonts.googleapis.com",
            // for monaco editor
            "cdn.jsdelivr.net",
            "'unsafe-inline'",
        ])
        .img_src(vec!["'self'", "data:"])
        .font_src(vec![
            "'self'",
            "fonts.gstatic.com",
            // for monaco editor
            "cdn.jsdelivr.net",
        ])
        .worker_src(vec!["'self'", "blob:"])
        .frame_src(vec!["'self'"]);

    // TODO: add HSTS

    app.layer(middleware::from_fn(set_csrf_cookie))
        .layer(CsrfLayer::new(csrf_config))
        .layer(HelmetLayer::new(
            Helmet::new()
                .add(XFrameOptions::same_origin())
                .add(XXSSProtection::on().mode_block())
                .add(ReferrerPolicy::no_referrer())
                .add(csp),
        ))
}
