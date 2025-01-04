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
pub async fn exercise_schema_list(
    State(state): State<AppState>,
    Cached(user): Cached<User>,
) -> Result<impl IntoResponse> {
    let mut txn = state
        .db()
        .begin()
        .await
        .wrap_err("Failed to start transaction")?;

    let schemas = exercise::get_exercise_schemas(&mut txn)
        .await
        .wrap_err("Failed to query exercise schemas")?;

    let inner = html! {
        div class="content" {
            div class="content__header" {
                h1 class="content__title" { "Database Schemas" }
                a href="/admin/exercise/schemas/new/" {
                    button class="button button--primary" {
                        i data-lucide="plus" class="button__icon" {}
                        "New Schema"
                    }
                }
            }
            div class="table-container" {
                table class="table" {
                    thead {
                        tr {
                            th class="table__header" { "Name" }
                            th class="table__header table__header--actions" { "Actions" }
                        }
                    }
                    tbody {
                        @for schema in &schemas {
                            tr class="table__row" {
                                td class="table__cell" { (schema.name()) }
                                td class="table__cell table__cell--actions" {
                                    button class="icon-button" {
                                        "View Exercises"
                                    }
                                    a href=(format!("/admin/exercise/schemas/{}/", schema.id())) {
                                        button class="icon-button" {
                                            i data-lucide="edit" class="icon-button__icon" {}
                                        }
                                    }
                                    button class="icon-button icon-button--danger" {
                                        i data-lucide="trash-2" class="icon-button__icon" {}
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
        "Exercise Schemas",
        app_layout(inner, "SQL Grimoire – Exercise Schemas", user.is_admin()),
    )
    .into_response())
}
