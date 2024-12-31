use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use sql_grimoire_id::define_id;

use super::Queryable;

define_id!(ExerciseSchemaId, "ex_schema");

#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct Exercise {
    id: ExerciseId,
    schema_id: ExerciseSchemaId,
    name: String,
    question: String,
    expected_query: String,
    expected_result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewExercise {
    pub schema_id: ExerciseSchemaId,
    pub name: String,
    pub question: String,
    pub expected_query: String,
    pub expected_result: String,
}

impl Exercise {
    pub fn update(&mut self, exercise: NewExercise) {
        self.schema_id = exercise.schema_id;
        self.name = exercise.name;
        self.question = exercise.question;
        self.expected_query = exercise.expected_query;
        self.expected_result = serde_json::from_str(&exercise.expected_result).unwrap();
    }
}

pub struct ExerciseInner {
    pub id: Uuid,
    pub schema_id: Uuid,
    pub name: String,
    pub question: String,
    pub expected_query: String,
    pub expected_result: serde_json::Value,
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
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct ExerciseListItem {
    id: ExerciseId,
    name: String,
    solved: bool,
}

pub struct ExerciseListItemInner {
    pub id: Uuid,
    pub name: String,
    pub solved: Option<bool>,
}

impl Queryable for ExerciseListItem {
    type Inner = ExerciseListItemInner;

    fn parse(inner: Self::Inner) -> Self {
        Self {
            id: inner.id.into(),
            name: inner.name,
            solved: inner.solved.unwrap_or(false),
        }
    }
}
