use axum::{
    debug_handler,
    extract::State,
    response::{IntoResponse, Redirect},
};
use eyre::WrapErr;
use maud::html;

use crate::{
    db::{exercise, user},
    error::Result,
    models::user::UserClaims,
    partials::{app_layout, page},
    state::WebAppState,
};

#[debug_handler]
#[tracing::instrument(skip_all)]
/// Main page of the web interface
pub async fn main_page(
    State(state): State<WebAppState>,
    user: UserClaims,
) -> Result<impl IntoResponse> {
    let mut txn = state
        .db()
        .begin()
        .await
        .wrap_err("Failed to start transaction")?;

    let user = match user::get_user(&mut txn, &user).await? {
        Some(user) => user,
        None => return Ok(Redirect::to("/login").into_response()),
    };

    let exercises = exercise::get_exercise_list(&mut txn, user.id()).await?;

    let inner = html! {
        div class="exercises" {
            div class="exercises__header" {
                h1 class="exercises__title" { "SQL Exercises" }
                @if user.is_admin() {
                    a href="/admin/exercise/new/" {
                        button class="button button--primary" {
                            i data-lucide="plus" class="button__icon" {}
                            "New Exercise"
                        }
                    }
                }
            }
            div class="exercises__container" {
                table class="exercises-table" {
                    thead {
                        tr {
                            th class="exercises-table__header" { "Exercise" }
                            th class="exercises-table__header exercises-table__header--center" { "Status" }
                            @if user.is_admin() {
                                th class="exercises-table__header exercises-table__header--actions" { "Actions" }
                            }
                        }
                    }
                    tbody {
                        @for exercise in &exercises {
                            tr class="exercises-table__row" {
                                td class="exercises-table__cell" {
                                    a href=(format!("/exercise/{}/", exercise.id())) class="exercise-link" {
                                        (exercise.name())
                                    }
                                }
                                td class="exercises-table__cell exercises-table__cell--center" {
                                    @if *exercise.solved() {
                                        i data-lucide="check-circle-2" class="status-icon status-icon--completed" {}
                                    } @else {
                                        span class="status-icon status-icon--pending" { "○" }
                                    }
                                }
                                @if user.is_admin() {
                                    td class="exercises-table__cell exercises-table__cell--actions" {
                                        a href=(format!("/admin/exercise/{}/", exercise.id())) {
                                            button class="icon-button" {
                                                i data-lucide="edit" class="icon-button__icon" {}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    Ok(page("SQL Grimoire", app_layout(inner, "SQL Grimoire")).into_response())
}
