-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS agent_id VARCHAR(255);
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS priority INTEGER;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS payload JSONB;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE shared_tasks DROP COLUMN IF EXISTS agent_id;
ALTER TABLE shared_tasks DROP COLUMN IF EXISTS priority;
ALTER TABLE shared_tasks DROP COLUMN IF EXISTS payload;
-- +goose StatementEnd
