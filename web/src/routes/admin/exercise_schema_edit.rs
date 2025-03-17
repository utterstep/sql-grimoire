use axum::{
    Json, debug_handler,
    extract::{Form, Path, State},
    http,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::Cached;
use eyre::WrapErr;
use maud::html;
use serde::Deserialize;

use crate::{
    db::exercise,
    error::{Result, SqlGrimoireError},
    models::{
        exercise::{ExerciseSchema, ExerciseSchemaId},
        user::User,
    },
    partials::{app_layout, page},
    state::AppState,
    static_files,
};

fn exercise_schema_form(exercise_schema: Option<ExerciseSchema>) -> maud::Markup {
    let name = exercise_schema
        .as_ref()
        .map(|schema| schema.name())
        .cloned()
        .unwrap_or_default();
    let schema = exercise_schema
        .as_ref()
        .map(|schema| schema.schema())
        .cloned()
        .unwrap_or_default();

    html! {
        form
            #db
            data-controller="db mermaid-schema-vis"
            data-action="db:db-created->mermaid-schema-vis#drawSchema"
            class="form"
            method="post"
        {
            h1 class="form__title" { "Edit Schema" }
            div class="form__group" {
                label class="form__label" { "Name" }
                input
                    name="name"
                    type="text"
                    class="form__input"
                    value=(name)
                    required
                    placeholder="Enter schema name";
            }
            div class="form__group" {
                label class="form__label" { "SQL Schema" }
                textarea
                    data-db-target="schema"
                    name="schema"
                    class="form__textarea"
                    placeholder="Enter SQL schema definition"
                    required
                    {
                        (schema)
                    }
            }

            div class="form__group" {
                label class="form__label" { "Schema Diagram" }
                div
                    #schema-vis
                    data-mermaid-schema-vis-target="schemaVis" {}
            }

            div class="form__actions" {
                a href="/admin/exercise/schemas/" class="button button--secondary" {
                    "Cancel"
                }
                button
                    data-action="db#resetDbRequest:prevent"
                    class="button button--secondary"
                {
                    "Draw Schema"
                }
                input
                    type="submit"
                    class="button button--primary"
                    value=(if exercise_schema.is_some() {
                        "Update Schema"
                    } else {
                        "Create Schema"
                    });
            }

            script type="module" src={"/static/" (static_files::db_controller.name)} {}
            script type="module" src={"/static/" (static_files::mermaid_schema_vis_controller.name)} {}
        }
    }
}

#[debug_handler]
#[tracing::instrument(skip_all)]
pub async fn exercise_schema_edit(
    State(state): State<AppState>,
    Path(id): Path<ExerciseSchemaId>,
    Cached(user): Cached<User>,
) -> Result<impl IntoResponse> {
    let mut conn = state
        .db()
        .acquire()
        .await
        .wrap_err("Failed to acquire DB connection")?;

    let exercise_schema = exercise::get_exercise_schema(&mut conn, id)
        .await
        .wrap_err("Failed to query exercise schema")?;

    let exercise_schema = match exercise_schema {
        Some(exercise_schema) => exercise_schema,
        None => {
            return Ok((http::StatusCode::NOT_FOUND, "Exercise schema not found").into_response());
        }
    };

    let form = exercise_schema_form(Some(exercise_schema));
    let inner = app_layout(
        html! {
            div class="content__header" {
                a href="/admin/exercise/schemas/" class="button button--text" {
                    i data-lucide="chevron-left" class="button__icon" {}
                    "Back to Schemas"
                }
            }
            (form)
        },
        "SQL Grimoire - Exercise Schema Edit",
        user.auth_state(),
    );

    Ok(page("SQL Grimoire - Exercise Schema Edit", inner).into_response())
}

#[debug_handler]
#[tracing::instrument(skip_all)]
pub async fn exercise_schema_new(
    State(_state): State<AppState>,
    Cached(user): Cached<User>,
) -> Result<impl IntoResponse> {
    let form = exercise_schema_form(None);
    let inner = app_layout(
        html! {
            div class="content__header" {
                a href="/admin/exercise/schemas/" class="button button--text" {
                    i data-lucide="chevron-left" class="button__icon" {}
                    "Back to Schemas"
                }
            }
            (form)
        },
        "SQL Grimoire - Exercise Schema New",
        user.auth_state(),
    );

    Ok(page("SQL Grimoire - Exercise Schema New", inner).into_response())
}

#[derive(Deserialize)]
pub struct ExerciseSchemaForm {
    name: String,
    schema: String,
}

#[debug_handler]
#[tracing::instrument(skip_all)]
pub async fn exercise_schema_post(
    State(state): State<AppState>,
    id: Option<Path<ExerciseSchemaId>>,
    Form(form): Form<ExerciseSchemaForm>,
) -> Result<impl IntoResponse> {
    let mut txn = state
        .db()
        .begin()
        .await
        .wrap_err("Failed to begin transaction")?;

    let id = {
        if let Some(id) = id {
            let mut exercise_schema = match exercise::get_exercise_schema(&mut txn, id.0)
                .await
                .wrap_err("Failed to query exercise schema")?
            {
                Some(exercise_schema) => exercise_schema,
                None => {
                    return Ok(
                        (http::StatusCode::NOT_FOUND, "Exercise schema not found").into_response()
                    );
                }
            };

            exercise_schema.set_name(form.name);
            exercise_schema.set_schema(form.schema);

            *exercise::update_exercise_schema(&mut txn, exercise_schema)
                .await
                .wrap_err("Failed to update exercise schema")?
                .id()
        } else {
            *exercise::create_exercise_schema(&mut txn, form.name, form.schema)
                .await
                .wrap_err("Failed to create exercise schema")?
                .id()
        }
    };

    txn.commit()
        .await
        .wrap_err("Failed to commit transaction")?;

    Ok(Redirect::to(&format!("/admin/exercise/schemas/{}/", id)).into_response())
}

#[debug_handler]
#[tracing::instrument(skip_all)]
pub async fn exercise_schema_json(
    State(state): State<AppState>,
    Path(id): Path<ExerciseSchemaId>,
) -> Result<impl IntoResponse> {
    let mut conn = state
        .db()
        .acquire()
        .await
        .wrap_err("Failed to acquire DB connection")?;

    let exercise_schema = exercise::get_exercise_schema(&mut conn, id)
        .await
        .wrap_err("Failed to query exercise schema")?;

    let exercise_schema = match exercise_schema {
        Some(exercise_schema) => exercise_schema,
        None => return Err(SqlGrimoireError::not_found("Exercise schema not found")),
    };

    Ok(Json(exercise_schema))
}
