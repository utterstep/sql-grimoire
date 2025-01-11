use axum::{debug_handler, extract::State, response::IntoResponse};
use axum_extra::extract::Cached;
use eyre::WrapErr;
use maud::html;

use crate::{
    db::exercise,
    error::Result,
    models::user::User,
    partials::{app_layout, page},
    state::AppState,
};

#[debug_handler]
#[tracing::instrument(skip_all)]
/// Main page of the web interface
pub async fn main_page(
    State(state): State<AppState>,
    Cached(user): Cached<User>,
) -> Result<impl IntoResponse> {
    let mut txn = state
        .db()
        .begin()
        .await
        .wrap_err("Failed to start transaction")?;

    let exercises = exercise::get_exercise_list(&mut txn, user.id())
        .await
        .wrap_err("Failed to get exercise list")?
        .into_iter()
        .filter(|ex| user.is_admin() || *ex.published());

    let inner = html! {
        div class="exercises" {
            div class="exercises__header" {
                h1 class="exercises__title" { "SQL Exercises" }
                @if user.is_admin() {
                    a href="/admin/exercise/new/" class="button button--primary" {
                        i data-lucide="plus" class="button__icon" {}
                        "New Exercise"
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
                                th class="exercises-table__header exercises-table__header--center" { "Published" }
                                th class="exercises-table__header exercises-table__header--actions" { "Actions" }
                            }
                        }
                    }
                    tbody {
                        @for exercise in exercises {
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
                                    td class="exercises-table__cell exercises-table__cell--center" {
                                        @if *exercise.published() {
                                            i data-lucide="check-circle-2" class="status-icon status-icon--completed" {}
                                        } @else {
                                            span class="status-icon status-icon--pending" { "○" }
                                        }
                                    }
                                    td class="exercises-table__cell exercises-table__cell--actions" {
                                        a href=(format!("/admin/exercise/{}/", exercise.id())) class="icon-button" {
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
    };

    Ok(page(
        "SQL Grimoire",
        app_layout(inner, "SQL Grimoire", user.is_admin()),
    )
    .into_response())
}
