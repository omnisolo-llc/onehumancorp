-- +goose Up
-- +goose StatementBegin
ALTER TABLE usage_events ADD COLUMN cached_tokens BIGINT NOT NULL DEFAULT 0;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE usage_events DROP COLUMN cached_tokens;
-- +goose StatementEnd
