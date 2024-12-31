use eyre::{Result, WrapErr};
use sqlx::postgres::PgConnection;

use sql_grimoire_id::Id;

use crate::models::{
    exercise::{
        Exercise, ExerciseId, ExerciseListItem, ExerciseSchema, ExerciseSchemaId,
        ExerciseSchemaListItem, NewExercise,
    },
    Queryable,
};

type ExerciseListItemInner = <ExerciseListItem as Queryable>::Inner;
type ExerciseInner = <Exercise as Queryable>::Inner;
type ExerciseSchemaListItemInner = <ExerciseSchemaListItem as Queryable>::Inner;
type ExerciseSchemaInner = <ExerciseSchema as Queryable>::Inner;

#[tracing::instrument(skip(conn))]
pub async fn get_exercise_schemas(conn: &mut PgConnection) -> Result<Vec<ExerciseSchemaListItem>> {
    let schemas = sqlx::query_as!(
        ExerciseSchemaListItemInner,
        "SELECT id, name FROM exercise_schema",
    )
    .fetch_all(conn)
    .await
    .wrap_err("Failed to get exercise schemas")?;

    Ok(schemas.into_iter().map(Queryable::parse).collect())
}

#[tracing::instrument(skip(conn))]
pub async fn get_exercise_schema(
    conn: &mut PgConnection,
    id: ExerciseSchemaId,
) -> Result<Option<ExerciseSchema>> {
    let schema = sqlx::query_as!(
        ExerciseSchemaInner,
        "SELECT id, name, schema FROM exercise_schema WHERE id = $1",
        id.get(),
    )
    .fetch_optional(conn)
    .await
    .wrap_err("Failed to get exercise schema")?
    .map(Queryable::parse);

    Ok(schema)
}

#[tracing::instrument(skip(conn))]
pub async fn create_exercise_schema(
    conn: &mut PgConnection,
    name: String,
    schema: String,
) -> Result<ExerciseSchema> {
    Ok(Queryable::parse(
        sqlx::query_as!(
            ExerciseSchemaInner,
            "INSERT INTO exercise_schema (name, schema) VALUES ($1, $2) RETURNING id, name, schema",
            name,
            schema,
        )
        .fetch_one(conn)
        .await
        .wrap_err("Failed to create exercise schema")?,
    ))
}

#[tracing::instrument(skip(conn))]
pub async fn update_exercise_schema(
    conn: &mut PgConnection,
    schema: ExerciseSchema,
) -> Result<ExerciseSchema> {
    Ok(Queryable::parse(
        sqlx::query_as!(
            ExerciseSchemaInner,
            "UPDATE exercise_schema SET name = $1, schema = $2 WHERE id = $3 RETURNING id, name, schema",
            schema.name(),
            schema.schema(),
            schema.id().get(),
        )
        .fetch_one(conn)
        .await
        .wrap_err("Failed to update exercise schema")?,
    ))
}

#[tracing::instrument(skip(conn))]
pub async fn get_exercise_list(
    conn: &mut PgConnection,
    user_id: &str,
) -> Result<Vec<ExerciseListItem>> {
    let exercises = sqlx::query_as!(
        ExerciseListItemInner,
        "SELECT
            exercise.id,
            exercise.name,
            'correct' = ANY(user_solution.status) AS solved
        FROM exercise
        LEFT OUTER JOIN (
            SELECT
                exercise_id,
                ARRAY_AGG(status) AS status
            FROM user_solution
            WHERE
                user_id = $1
            GROUP BY exercise_id
        ) AS user_solution ON exercise.id = user_solution.exercise_id
        ORDER BY exercise.id
        ",
        user_id,
    )
    .fetch_all(conn)
    .await
    .wrap_err("Failed to get exercise list")?;

    Ok(exercises.into_iter().map(Queryable::parse).collect())
}

#[tracing::instrument(skip(conn))]
pub async fn get_exercise(conn: &mut PgConnection, id: ExerciseId) -> Result<Option<Exercise>> {
    let exercise = sqlx::query_as!(
        ExerciseInner,
        "SELECT id, name, schema_id, question, expected_query, expected_result
        FROM exercise
        WHERE id = $1",
        id.get(),
    )
    .fetch_optional(conn)
    .await
    .wrap_err("Failed to get exercise")?
    .map(Queryable::parse);

    Ok(exercise)
}

#[tracing::instrument(skip(conn))]
pub async fn create_exercise(conn: &mut PgConnection, exercise: NewExercise) -> Result<Exercise> {
    Ok(Queryable::parse(
        sqlx::query_as!(
            ExerciseInner,
            "INSERT INTO exercise
                (schema_id, name, question, expected_query, expected_result)
            VALUES
                ($1, $2, $3, $4, $5)
            RETURNING
                id, schema_id, name, question, expected_query, expected_result",
            exercise.schema_id.get(),
            exercise.name,
            exercise.question,
            exercise.expected_query,
            serde_json::from_str::<serde_json::Value>(&exercise.expected_result)
                .wrap_err("invalid JSON in expected result")?,
        )
        .fetch_one(conn)
        .await
        .wrap_err("Failed to create exercise")?,
    ))
}

#[tracing::instrument(skip(conn))]
pub async fn update_exercise(conn: &mut PgConnection, exercise: Exercise) -> Result<Exercise> {
    Ok(Queryable::parse(
        sqlx::query_as!(
            ExerciseInner,
            "UPDATE exercise
            SET
                name = $1,
                question = $2,
                expected_query = $3,
                expected_result = $4
            WHERE
                id = $5
            RETURNING
                id, schema_id, name, question, expected_query, expected_result",
            exercise.name(),
            exercise.question(),
            exercise.expected_query(),
            exercise.expected_result(),
            exercise.id().get(),
        )
        .fetch_one(conn)
        .await
        .wrap_err("Failed to update exercise")?,
    ))
}
