use derive_getters::{Dissolve, Getters};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use sql_grimoire_id::define_id;

use super::Queryable;

define_id!(ExerciseSchemaId, "ex_schema");

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Dissolve)]
pub struct ExerciseSchema {
    id: ExerciseSchemaId,
    name: String,
    schema: String,
}

impl ExerciseSchema {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_schema(&mut self, schema: String) {
        self.schema = schema;
    }
}

pub struct ExerciseSchemaInner {
    pub id: Uuid,
    pub name: String,
    pub schema: String,
}

impl Queryable for ExerciseSchema {
    type Inner = ExerciseSchemaInner;

    fn parse(inner: Self::Inner) -> Self {
        Self {
            id: inner.id.into(),
            name: inner.name,
            schema: inner.schema,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct ExerciseSchemaListItem {
    id: ExerciseSchemaId,
    name: String,
}

pub struct ExerciseSchemaListItemInner {
    pub id: Uuid,
    pub name: String,
}

impl Queryable for ExerciseSchemaListItem {
    type Inner = ExerciseSchemaListItemInner;

    fn parse(inner: Self::Inner) -> Self {
        Self {
            id: inner.id.into(),
            name: inner.name,
        }
    }
}

define_id!(ExerciseId, "ex");

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Dissolve)]
pub struct Exercise {
    id: ExerciseId,
    schema_id: ExerciseSchemaId,
    name: String,
    question: String,
    expected_query: String,
    expected_result: serde_json::Value,
    published_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Dissolve)]
pub struct NewExercise {
    schema_id: ExerciseSchemaId,
    name: String,
    question: String,
    expected_query: String,
    expected_result: String,
    published_at: Option<OffsetDateTime>,
}

impl Exercise {
    pub fn update(&mut self, new_exercise: NewExercise) {
        self.schema_id = new_exercise.schema_id;
        self.name = new_exercise.name;
        self.question = new_exercise.question;
        self.expected_query = new_exercise.expected_query;
        self.expected_result = serde_json::from_str(&new_exercise.expected_result).unwrap();
        self.published_at = new_exercise.published_at;
    }
}

pub struct ExerciseInner {
    pub id: Uuid,
    pub schema_id: Uuid,
    pub name: String,
    pub question: String,
    pub expected_query: String,
    pub expected_result: serde_json::Value,
    pub published_at: Option<OffsetDateTime>,
}

impl Queryable for Exercise {
    type Inner = ExerciseInner;

    fn parse(inner: Self::Inner) -> Self {
        Self {
            id: inner.id.into(),
            schema_id: inner.schema_id.into(),
            name: inner.name,
            question: inner.question,
            expected_query: inner.expected_query,
            expected_result: inner.expected_result,
            published_at: inner.published_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct ExerciseListItem {
    id: ExerciseId,
    name: String,
    solved: bool,
    published: bool,
}

pub struct ExerciseListItemInner {
    pub id: Uuid,
    pub name: String,
    pub solved: Option<bool>,
    pub published: bool,
}

impl Queryable for ExerciseListItem {
    type Inner = ExerciseListItemInner;

    fn parse(inner: Self::Inner) -> Self {
        Self {
            id: inner.id.into(),
            name: inner.name,
            solved: inner.solved.unwrap_or(false),
            published: inner.published,
        }
    }
}
