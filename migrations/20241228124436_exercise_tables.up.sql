-- Add up migration script here

CREATE TABLE exercise_schema (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v7(),
    name text NOT NULL,
    schema text NOT NULL,
    created_at timestamp NOT NULL DEFAULT now(),
    updated_at timestamp NOT NULL DEFAULT now()
);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON exercise_schema
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

CREATE TABLE exercise (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v7(),
    schema_id uuid NOT NULL REFERENCES exercise_schema(id),
    name text NOT NULL,
    question text NOT NULL,
    expected_query text NOT NULL,
    expected_result jsonb NOT NULL,

    created_at timestamp NOT NULL DEFAULT now(),
    updated_at timestamp NOT NULL DEFAULT now()
);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON exercise
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

