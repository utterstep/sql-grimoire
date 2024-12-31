use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use sql_grimoire_id::define_id;

use super::{exercise::ExerciseId, Queryable};

define_id!(UserSolutionId, "us");

#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct UserSolution {
    id: UserSolutionId,
    user_id: String,
    exercise_id: ExerciseId,
    query: String,
    result: Value,
    status: String,
}

pub struct UserSolutionInner {
    pub id: Uuid,
    pub user_id: String,
    pub exercise_id: Uuid,
    pub query: String,
    pub result: Value,
    pub status: String,
}

impl Queryable for UserSolution {
    type Inner = UserSolutionInner;

    fn parse(inner: Self::Inner) -> Self {
        Self {
            id: inner.id.into(),
            user_id: inner.user_id,
            exercise_id: inner.exercise_id.into(),
            query: inner.query,
            result: inner.result,
            status: inner.status,
        }
    }
}
