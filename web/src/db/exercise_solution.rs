use eyre::{Result, WrapErr};
use serde_json::Value;
use sqlx::postgres::PgConnection;

use sql_grimoire_id::Id;

use crate::models::{
    exercise::ExerciseId, exercise_solution::UserSolution, user::UserClaims, Queryable,
};

type UserSolutionInner = <UserSolution as Queryable>::Inner;

#[tracing::instrument(skip(conn))]
pub async fn get_last_user_solution(
    conn: &mut PgConnection,
    user: &UserClaims,
    exercise_id: ExerciseId,
) -> Result<Option<UserSolution>> {
    let solution = sqlx::query_as!(
        UserSolutionInner,
        "SELECT id, user_id, exercise_id, query, result, status
        FROM user_solution
        WHERE user_id = $1 AND exercise_id = $2
        ORDER BY created_at DESC
        LIMIT 1",
        user.sub(),
        exercise_id.get(),
    )
    .fetch_optional(conn)
    .await
    .wrap_err("Failed to get user solution")?
    .map(Queryable::parse);

    Ok(solution)
}

#[tracing::instrument(skip(conn))]
pub async fn create_user_solution(
    conn: &mut PgConnection,
    user_id: &str,
    exercise_id: ExerciseId,
    query: &str,
    result: Value,
    status: &str,
) -> Result<UserSolution> {
    Ok(Queryable::parse(
        sqlx::query_as!(
            UserSolutionInner,
            "INSERT INTO user_solution (user_id, exercise_id, query, result, status)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, exercise_id, query, result, status",
            user_id,
            exercise_id.get(),
            query,
            result,
            status,
        )
        .fetch_one(conn)
        .await
        .wrap_err("Failed to create user solution")?,
    ))
}

#[tracing::instrument(skip(conn))]
pub async fn update_user_solution(
    conn: &mut PgConnection,
    solution: UserSolution,
) -> Result<UserSolution> {
    Ok(Queryable::parse(
        sqlx::query_as!(
            UserSolutionInner,
            "UPDATE user_solution
            SET query = $1, result = $2, status = $3
            WHERE id = $4
            RETURNING id, user_id, exercise_id, query, result, status",
            solution.query(),
            solution.result(),
            solution.status(),
            solution.id().get(),
        )
        .fetch_one(conn)
        .await
        .wrap_err("Failed to update user solution")?,
    ))
}
