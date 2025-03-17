use axum::{
    debug_handler,
    extract::{Json, Path, State},
    http,
    response::IntoResponse,
};
use axum_extra::extract::Cached;
use eyre::{OptionExt, WrapErr};
use maud::{PreEscaped, html};
use serde::Deserialize;

use crate::{
    db::{exercise, exercise_solution},
    error::Result,
    models::{
        exercise::ExerciseId,
        user::{User, UserClaims},
    },
    partials::{app_layout, page},
    state::AppState,
    static_files,
};

#[debug_handler]
#[tracing::instrument(skip_all)]
pub async fn run(
    State(state): State<AppState>,
    Path(exercise_id): Path<ExerciseId>,
    Cached(user): Cached<User>,
    user_claims: UserClaims,
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

    let schema = exercise::get_exercise_schema(&mut conn, *exercise.schema_id())
        .await
        .wrap_err("Failed to query exercise schema")?
        .ok_or_eyre("Exercise schema not found")?;

    let solution = exercise_solution::get_last_user_solution(&mut conn, &user_claims, exercise_id)
        .await
        .wrap_err("Failed to query user solution")?;

    let solution_correct = solution
        .as_ref()
        .map(|s| s.status() == "correct")
        .unwrap_or(false);

    let title = format!("SQL Grimoire - {}", exercise.name());

    let question_text = {
        let question_parsed = pulldown_cmark::Parser::new(exercise.question());
        let mut question_html = String::new();
        pulldown_cmark::html::push_html(&mut question_html, question_parsed);

        PreEscaped(question_html)
    };

    let inner = app_layout(
        html! {
            div class="content__header" {
                a href="/" class="button button--text" {
                    i data-lucide="chevron-left" class="button__icon" {}
                    "Back to Exercises"
                }
            }
            turbo-frame
                #db
                data-controller="sql-run solution-submit sql-highlight mermaid-schema-vis db"
                data-action="db:db-created->mermaid-schema-vis#drawSchema"
                data-sql-run-editor-outlet="#editor"
                data-sql-run-db-outlet="#db"
                data-solution-submit-editor-outlet="#editor"
                data-solution-submit-db-outlet="#db"
            {
                script type="module" src={"/static/" (static_files::sql_highlight_controller.name)} {}

                div class="grid" {
                    div class="panel panel--exercise" {
                        h2 class="panel__title" { (exercise.name()) }
                        div class="panel__content" {
                            div class="panel__text" { (question_text) }
                            div
                                class="table-info"
                            {
                                details class="table-info__details" {
                                    summary class="table-info__summary" {
                                        h3 class="table-info__title" { "Database Definitions" }
                                    }
                                    pre class="table-info__schema" {
                                        code
                                            class="language-pgsql"
                                            data-sql-highlight-target="code"
                                        {
                                            (schema.schema())
                                        }
                                    }
                                    code
                                        class="hidden"
                                        data-db-target="schema"
                                    {
                                        (schema.schema())
                                    }
                                }
                            }
                        }
                    }

                    div
                        #editor
                        class="panel panel--editor"
                        data-action="db:db-created@window->editor#updateSchemaSuggestions"
                        data-controller="editor"
                        data-editor-mode-value="monaco"
                    {
                        div class="editor__header" {
                            h3 class="editor__title" { "Query Editor" }

                            div
                                class="editor__actions"
                                data-controller="drag-resize"
                            {
                                input
                                    type="checkbox"
                                    id="schema-toggle"
                                    class="schema-toggle"
                                    data-action="drag-resize#resetWidth"
                                {}

                                label
                                    for="schema-toggle"
                                    class="button button--secondary"
                                {
                                    i data-lucide="table-properties" class="button__icon" {}
                                    span { "Show Schema" }
                                }

                                div
                                    data-drag-resize-target="resizable"
                                    class="schema-sidebar"
                                {
                                    div
                                        class="schema-sidebar__draghandle"
                                        data-drag-resize-target="handle" {}

                                    div class="schema-sidebar__content" {
                                        div class="schema-sidebar__header" {
                                            h2 class="schema-sidebar__title" { (schema.name()) }
                                        }

                                        div
                                            class="schema-sidebar__body"
                                            data-mermaid-schema-vis-target="schemaVis"
                                        {}
                                    }
                                }

                                button
                                    class="button button--secondary"
                                    data-action="sql-run#execute"
                                {
                                    i data-lucide="database" class="button__icon" {}
                                    span { "Execute Query" }
                                }
                                button
                                    class="button button--primary"
                                    data-action="solution-submit#submit"
                                {
                                    i data-lucide="send" class="button__icon" {}
                                    span { "Submit Solution" }
                                }
                            }
                        }

                        div
                            class="editor__textarea"
                            placeholder="Write your SQL query here..."
                            data-language="pgsql"
                            data-editor-target="editor"
                        {
                            (solution.as_ref().map(|s| s.query().as_str()).unwrap_or(""))
                        }

                        @if let Some(solution) = &solution {
                            @let solution_status = solution.status();

                            div class={"solution-status solution-status--" (solution_status)} {
                                @if solution_status == "correct" {
                                    i data-lucide="check" class="solution-status__icon" {}
                                    span { "Correct! Well done!" }
                                } @else {
                                    i data-lucide="alert-circle" class="solution-status__icon" {}
                                    span { "Not quite right. Try again!" }
                                }
                            }
                        }

                        @if solution_correct {
                            div class="expected-query" {
                                div class="expected-query__header" {
                                    i data-lucide="check" class="expected-query__icon" {}
                                    span class="expected-query__title" { "Expected Query" }
                                }
                                pre class="expected-query__code" {
                                    code
                                        class="language-sql"
                                        data-sql-highlight-target="code"
                                    {
                                        (exercise.expected_query())
                                    }
                                }
                            }
                        }
                    }
                }

                div class="panel panel--results" {
                    h3 class="panel__title" { "Query Results" }

                    div
                        class="results-table"
                        data-sql-run-target="results"
                    {
                        "Run the query to see the results"
                    }
                }
            }

            script type="module" src={"/static/" (static_files::db_controller.name)} {}
            script type="module" src={"/static/" (static_files::mermaid_schema_vis_controller.name)} {}
            script type="module" src={"/static/" (static_files::drag_resize_controller.name)} {}

            script defer type="module" src={"/static/" (static_files::monaco_init.name)} {}
            script defer type="module" src={"/static/" (static_files::editor_controller.name)} {}
            script defer type="module" src={"/static/" (static_files::sql_run_controller.name)} {}
            script defer type="module" src={"/static/" (static_files::solution_submit_controller.name)} {}
        },
        exercise.name(),
        user.auth_state(),
    );

    Ok(page(&title, inner).into_response())
}

#[derive(Debug, Deserialize)]
pub struct ExerciseCheckResultRequest {
    query: String,
    result: serde_json::Value,
}

#[debug_handler]
#[tracing::instrument(skip_all)]
pub async fn submit_solution(
    State(state): State<AppState>,
    Path(exercise_id): Path<ExerciseId>,
    user: UserClaims,
    Json(results): Json<ExerciseCheckResultRequest>,
) -> Result<impl IntoResponse> {
    let mut txn = state
        .db()
        .begin()
        .await
        .wrap_err("Failed to start transaction")?;

    let exercise = exercise::get_exercise(&mut txn, exercise_id)
        .await
        .wrap_err("Failed to query exercise")?
        .ok_or_eyre("Exercise not found")?;

    let status = {
        if exercise.expected_result() == &results.result {
            "correct"
        } else {
            "incorrect"
        }
    };

    let solution_id = *exercise_solution::create_user_solution(
        &mut txn,
        user.sub(),
        exercise_id,
        &results.query,
        results.result,
        status,
    )
    .await
    .wrap_err("Failed to create user solution")?
    .id();

    txn.commit()
        .await
        .wrap_err("Failed to commit transaction")?;

    Ok((
        http::StatusCode::CREATED,
        Json(serde_json::json!({ "solution_id": solution_id })),
    ))
}
