use derive_getters::Getters;
use serde::{Deserialize, Deserializer, Serialize};

use super::Queryable;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    User,
    Unknown(String),
}

impl From<String> for UserRole {
    fn from(value: String) -> Self {
        match value.as_str() {
            "admin" => UserRole::Admin,
            "user" => UserRole::User,
            _ => UserRole::Unknown(value),
        }
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "admin"),
            UserRole::User => write!(f, "user"),
            UserRole::Unknown(value) => write!(f, "{}", value),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct User {
    id: String,
    role: UserRole,
}

impl User {
    pub fn is_admin(&self) -> bool {
        self.role == UserRole::Admin
    }
}

pub struct UserInner {
    pub id: String,
    pub role: UserRole,
}

impl Queryable for User {
    type Inner = UserInner;

    fn parse(inner: Self::Inner) -> Self {
        Self {
            id: inner.id,
            role: inner.role,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct UserClaims {
    iss: String,
    sub: String,
    #[serde(deserialize_with = "deserialize_datetime_from_timestamp")]
    exp: time::OffsetDateTime,
    #[serde(deserialize_with = "deserialize_datetime_from_timestamp")]
    nbf: time::OffsetDateTime,
    email: String,
}

fn deserialize_datetime_from_timestamp<'de, D>(
    deserializer: D,
) -> Result<time::OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp = i64::deserialize(deserializer)?;

    time::OffsetDateTime::from_unix_timestamp(timestamp).map_err(serde::de::Error::custom)
}
