-- +goose postgres
-- +goose Up
-- +goose StatementBegin
ALTER TABLE sub_agent_queue ADD COLUMN IF NOT EXISTS attempts INTEGER DEFAULT 0;
ALTER TABLE sub_agent_queue ADD COLUMN IF NOT EXISTS max_attempts INTEGER DEFAULT 3;
ALTER TABLE sub_agent_queue ADD COLUMN IF NOT EXISTS run_after TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- +goose StatementEnd
