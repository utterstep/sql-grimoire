use axum::{
    debug_handler,
    extract::{Form, Path, State},
    http,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::Cached;
use eyre::WrapErr;
use maud::{html, Markup};

use crate::{
    db::exercise,
    error::Result,
    models::{
        exercise::{Exercise, ExerciseId, ExerciseSchemaListItem, NewExercise},
        user::User,
    },
    partials::{app_layout, page},
    state::AppState,
    static_files,
};

fn exercise_form(exercise: Option<&Exercise>, schemas: &[ExerciseSchemaListItem]) -> Markup {
    let title = match exercise {
        Some(exercise) => format!("Edit Exercise \"{}\"", exercise.name()),
        None => "New Exercise".to_string(),
    };

    let submit_text = if exercise.is_some() {
        "Update Exercise"
    } else {
        "Create Exercise"
    };

    html! {
        form
            data-controller="sql-run schema-hidden"
            data-action="schema-hidden:schema-updated->sql-run#schemaUpdated"
            data-sql-run-editor-outlet="#editor"
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
                        data-action="schema-hidden#processChange"
                    {
                        @for schema in schemas {
                            option
                                value=(schema.id())
                                selected=(exercise.map(|ex| ex.schema_id() == schema.id()).unwrap_or(false))
                            {
                                (schema.name())
                            }
                        }
                    }
                }

                div
                    data-sql-run-target="schema"
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
                a href="/admin/exercise/" {
                    button class="button button--secondary" {
                        "Cancel"
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

    let form = exercise_form(Some(&exercise), &schemas);

    let inner = app_layout(
        html! {
            div class="content__header" {
                a href="/" {
                    button class="button button--text" {
                        i data-lucide="chevron-left" class="button__icon" {}
                        "Back to Exercises"
                    }
                }
            }
            (form)
        },
        "SQL Grimoire - Exercise Edit",
        user.is_admin(),
    );

    Ok(page("SQL Grimoire - Exercise Edit", inner).into_response())
}

#[debug_handler]
#[tracing::instrument(skip_all)]
pub async fn exercise_new(
    State(state): State<AppState>,
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

    let form = exercise_form(None, &schemas);

    let inner = app_layout(
        html! {
            div class="content__header" {
                a href="/" {
                    button class="button button--text" {
                        i data-lucide="chevron-left" class="button__icon" {}
                        "Back to Exercises"
                    }
                }
            }
            (form)
        },
        "SQL Grimoire - New Exercise",
        user.is_admin(),
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
