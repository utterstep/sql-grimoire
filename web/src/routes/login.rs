use axum::{
    debug_handler,
    extract::State,
    response::{IntoResponse, Redirect},
};
use eyre::WrapErr;
use maud::{Markup, html};
use tracing::debug;

use crate::{
    db::user,
    error::Result,
    models::user::UserClaims,
    partials::{AuthState, app_layout, page},
    state::AppState,
    static_files::corbado_login,
};

#[debug_handler]
#[tracing::instrument]
pub async fn login() -> Markup {
    let inner = app_layout(
        html! {
            script src={"/static/" (corbado_login.name)} {}
            div #corbado-auth {}
        },
        "Login",
        AuthState::Unauthenticated,
    );

    page("Login", inner)
}

#[debug_handler]
#[tracing::instrument(skip_all, fields(sub = %claims.sub()))]
pub async fn after_login(
    State(state): State<AppState>,
    claims: UserClaims,
) -> Result<impl IntoResponse> {
    let mut txn = state
        .db()
        .begin()
        .await
        .wrap_err("Failed to begin transaction")?;

    let user = user::get_user(&mut txn, &claims).await?;

    if user.is_none() {
        debug!("Creating new user");
        user::create_user(&mut txn, &claims).await?;
    }

    txn.commit()
        .await
        .wrap_err("Failed to commit transaction")?;

    Ok(Redirect::to("/"))
}
