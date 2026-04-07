-- +goose Up
ALTER TABLE agent_missions ADD COLUMN updated_at DATETIME;
UPDATE agent_missions SET updated_at = created_at WHERE updated_at IS NULL;

-- +goose Down
