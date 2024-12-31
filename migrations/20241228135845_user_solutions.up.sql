-- Add up migration script here
CREATE TABLE user_solution (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id text NOT NULL REFERENCES users(id),

    exercise_id uuid NOT NULL REFERENCES exercise(id),
    query text NOT NULL,
    result jsonb NOT NULL,
    status text NOT NULL,

    created_at timestamp NOT NULL DEFAULT now(),
    updated_at timestamp NOT NULL DEFAULT now()
);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON user_solution
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- composite index on user_id and exercise_id
CREATE INDEX user_solution_user_exercise_idx ON user_solution (user_id, exercise_id);
