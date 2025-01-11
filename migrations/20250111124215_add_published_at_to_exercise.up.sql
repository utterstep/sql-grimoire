-- Add up migration script here
ALTER TABLE exercise ADD COLUMN published_at timestamp with time zone;

-- migrate existing exercises to published_at
UPDATE exercise SET published_at = created_at;
