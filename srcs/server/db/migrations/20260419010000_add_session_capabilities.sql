-- +goose Up
-- +goose StatementBegin
ALTER TABLE agent_session_data ADD COLUMN IF NOT EXISTS capabilities JSONB DEFAULT '[]'::jsonb;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE agent_session_data DROP COLUMN IF EXISTS capabilities;
-- +goose StatementEnd
