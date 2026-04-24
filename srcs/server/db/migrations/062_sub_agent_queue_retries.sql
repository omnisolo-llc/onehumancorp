-- +goose Up
-- +goose StatementBegin
ALTER TABLE sub_agent_queue ADD COLUMN attempts INTEGER NOT NULL DEFAULT 0;
ALTER TABLE sub_agent_queue ADD COLUMN max_attempts INTEGER NOT NULL DEFAULT 3;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE sub_agent_queue DROP COLUMN attempts;
ALTER TABLE sub_agent_queue DROP COLUMN max_attempts;
-- +goose StatementEnd
