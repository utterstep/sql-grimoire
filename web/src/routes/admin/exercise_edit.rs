use axum::{
    debug_handler,
    extract::{Form, Path, Query, State},
    http,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::Cached;
use eyre::WrapErr;
use maud::{Markup, html};
use serde::Deserialize;
use time::OffsetDateTime;

use crate::{
    db::exercise,
    error::Result,
    models::{
        exercise::{Exercise, ExerciseId, ExerciseSchemaId, ExerciseSchemaListItem, NewExercise},
        user::User,
    },
    partials::{app_layout, page},
    state::AppState,
    static_files,
};

#[derive(Debug, Deserialize)]
pub struct NewExerciseQuery {
    schema_id: Option<ExerciseSchemaId>,
}

fn exercise_form(
    exercise: Option<&Exercise>,
    schemas: &[ExerciseSchemaListItem],
    schema_preselected: Option<ExerciseSchemaId>,
) -> Markup {
    let title = match exercise {
        Some(exercise) => format!("Editing Exercise \"{}\"", exercise.name()),
        None => "New Exercise".to_string(),
    };

    let selected_schema_id = exercise.map(|e| *e.schema_id()).or(schema_preselected);

    let submit_text = if exercise.is_some() {
        "Update Exercise"
    } else {
        "Create Exercise"
    };

    let published_at = exercise
        // either the already existing published_at
        .and_then(|ex| *ex.published_at())
        // or the current time
        .unwrap_or_else(OffsetDateTime::now_utc);

    html! {
        form
            #db
            data-controller="sql-run schema-hidden db"
            data-action="schema-hidden:schema-updated->db#schemaUpdated"
            data-sql-run-editor-outlet="#editor"
            data-sql-run-db-outlet="#db"
            class="form"
            method="post"
        {
            h1 class="form__title" { (title) }

            div class="form__group" {
                label class="form__label" { "Database Schema" }
                div class="select-wrapper" {
                    select
                        class="form__select"
                        name="schema_id"
                        required
                        data-schema-hidden-target="schemaSelector"
                        data-action="schema-hidden#fetchSchema"
                    {
                        @for schema in schemas {
                            option
                                value=(schema.id())
                                selected[selected_schema_id.map(|id| &id == schema.id()).unwrap_or(false)]
                            {
                                (schema.name())
                            }

                        }
                    }
                }

                div
                    data-db-target="schema"
                    data-schema-hidden-target="schemaHidden"
                    class="hidden" {}
            }

            div class="form__group" {
                label class="form__label" { "Exercise Name" }
                input
                    type="text"
                    class="form__input"
                    name="name"
                    placeholder="Enter exercise name"
                    required
                    value=(exercise.map(|ex| ex.name().to_owned()).unwrap_or_default())
                ;
            }

            div class="form__group" {
                label class="form__label" { "Question" }
                textarea
                    class="form__textarea"
                    name="question"
                    placeholder="Enter exercise question"
                    required
                    {
                        (exercise.map(|ex| ex.question().to_owned()).unwrap_or_default())
                    }
                ;
            }

            div class="form__group" {
                label class="form__label" { "Published" }
                input
                    type="checkbox"
                    name="published_at"
                    checked[exercise.map(|ex| ex.published_at().is_some()).unwrap_or(false)]
                    value=(published_at)
                {}
            }

            div class="form__group"
            {
                label class="form__label" { "Expected Query" }
                div
                    #editor
                    class="query-editor"
                    data-controller="editor"
                    data-editor-mode-value="simple"
                {
                    textarea
                        class="form__textarea"
                        name="expected_query"
                        placeholder="Enter expected SQL query"
                        data-editor-target="editor"
                        required
                        {
                            (exercise.map(|ex| ex.expected_query().to_owned()).unwrap_or_default())
                        }
                    ;
                }
            }

            button
                class="button button--primary"
                data-action="sql-run#executeToTextArea"
            {
                i data-lucide="database" class="button__icon" {}
                span { "Execute Query" }
            }

            div class="form__group" {
                label class="form__label" { "Expected Result (JSON)" }
                textarea
                    class="form__textarea"
                    name="expected_result"
                    placeholder="Enter expected result as JSON"
                    data-sql-run-target="results"
                    required
                    {
                        (exercise.map(|ex| ex.expected_result().to_owned()).unwrap_or_default())
                    }
                ;
            }

            div class="form__actions" {
                a
                    class="button button--secondary"
                    href="/"
                {
                    "Cancel"
                }
                @if let Some(ex) = exercise {
                    a
                        class="button button--secondary"
                        href={"/admin/exercise/new/?schema_id=" (ex.schema_id())}
                    {
                        "Create new"
                    }
                }
                input
                    type="submit"
                    class="button button--primary"
                    value=(submit_text);
            }
        }

        script type="module" src={"/static/" (static_files::schema_hidden_controller.name)} {}
        script type="module" src={"/static/" (static_files::editor_controller.name)} {}
        script defer type="module" src={"/static/" (static_files::db_controller.name)} {}
        script defer type="module" src={"/static/" (static_files::sql_run_controller.name)} {}
    }
}

#[debug_handler]
#[tracing::instrument(skip_all)]
pub async fn exercise_edit(
    State(state): State<AppState>,
    Path(exercise_id): Path<ExerciseId>,
    Cached(user): Cached<User>,
) -> Result<impl IntoResponse> {
    let mut conn = state
        .db()
        .acquire()
        .await
        .wrap_err("Failed to acquire DB connection")?;

    let exercise = match exercise::get_exercise(&mut conn, exercise_id)
        .await
        .wrap_err("Failed to query exercise")?
    {
        Some(exercise) => exercise,
        None => return Ok((http::StatusCode::NOT_FOUND, "Exercise not found").into_response()),
    };

    let schemas = exercise::get_exercise_schemas(&mut conn)
        .await
        .wrap_err("Failed to query exercise schemas")?;

    let form = exercise_form(Some(&exercise), &schemas, None);

    let inner = app_layout(
        html! {
            div class="content__header" {
                a class="button button--text" href="/" {
                    i data-lucide="chevron-left" class="button__icon" {}
                    "Back to Exercises"
                }
            }
            (form)
        },
        "SQL Grimoire - Exercise Edit",
        user.auth_state(),
    );

    Ok(page("SQL Grimoire - Exercise Edit", inner).into_response())
}

#[debug_handler]
#[tracing::instrument(skip_all)]
pub async fn exercise_new(
    State(state): State<AppState>,
    Query(schema_preselected): Query<NewExerciseQuery>,
    Cached(user): Cached<User>,
) -> Result<impl IntoResponse> {
    let mut conn = state
        .db()
        .acquire()
        .await
        .wrap_err("Failed to acquire DB connection")?;

    let schemas = exercise::get_exercise_schemas(&mut conn)
        .await
        .wrap_err("Failed to query exercise schemas")?;

    let form = exercise_form(None, &schemas, schema_preselected.schema_id);

    let inner = app_layout(
        html! {
            div class="content__header" {
                a href="/" class="button button--text" {
                    i data-lucide="chevron-left" class="button__icon" {}
                    "Back to Exercises"
                }
            }
            (form)
        },
        "SQL Grimoire - New Exercise",
        user.auth_state(),
    );

    Ok(page("SQL Grimoire - New Exercise", inner).into_response())
}

#[debug_handler]
#[tracing::instrument(skip_all)]
pub async fn exercise_post(
    State(state): State<AppState>,
    id: Option<Path<ExerciseId>>,
    Form(form): Form<NewExercise>,
) -> Result<impl IntoResponse> {
    let mut txn = state
        .db()
        .begin()
        .await
        .wrap_err("Failed to begin transaction")?;

    let id = {
        if let Some(id) = id {
            let mut exercise = match exercise::get_exercise(&mut txn, id.0)
                .await
                .wrap_err("Failed to query exercise")?
            {
                Some(exercise) => exercise,
                None => {
                    return Ok(http::StatusCode::NOT_FOUND.into_response());
                }
            };

            exercise.update(form);

            *exercise::update_exercise(&mut txn, exercise)
                .await
                .wrap_err("Failed to update exercise")?
                .id()
        } else {
            *exercise::create_exercise(&mut txn, form)
                .await
                .wrap_err("Failed to create exercise")?
                .id()
        }
    };

    txn.commit()
        .await
        .wrap_err("Failed to commit transaction")?;

    Ok(Redirect::to(&format!("/admin/exercise/{}/", id)).into_response())
}
