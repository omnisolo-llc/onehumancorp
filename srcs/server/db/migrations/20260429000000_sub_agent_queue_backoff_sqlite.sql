-- +goose sqlite3
-- +goose Up
-- +goose StatementBegin
ALTER TABLE sub_agent_queue ADD COLUMN attempts INTEGER DEFAULT 0;
ALTER TABLE sub_agent_queue ADD COLUMN max_attempts INTEGER DEFAULT 3;
ALTER TABLE sub_agent_queue ADD COLUMN run_after DATETIME DEFAULT CURRENT_TIMESTAMP;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- +goose StatementEnd
