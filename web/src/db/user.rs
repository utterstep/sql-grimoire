use eyre::{Result, WrapErr};
use sqlx::postgres::PgConnection;

use crate::models::{
    Queryable,
    user::{User, UserClaims, UserRole},
};

type UserInner = <User as Queryable>::Inner;

#[tracing::instrument(skip(conn))]
pub async fn create_user(conn: &mut PgConnection, user: &UserClaims) -> Result<User> {
    let user = sqlx::query_as!(
        UserInner,
        "INSERT INTO users (id, role) VALUES ($1, $2) RETURNING id, role",
        user.sub(),
        UserRole::User.to_string(),
    )
    .fetch_one(conn)
    .await
    .wrap_err("Failed to create user")
    .map(Queryable::parse)?;

    Ok(user)
}

#[tracing::instrument(skip(conn))]
pub async fn get_user(conn: &mut PgConnection, user: &UserClaims) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        UserInner,
        "SELECT id, role FROM users WHERE id = $1",
        user.sub(),
    )
    .fetch_optional(conn)
    .await
    .wrap_err("Failed to get user")?
    .map(Queryable::parse);

    Ok(user)
}
