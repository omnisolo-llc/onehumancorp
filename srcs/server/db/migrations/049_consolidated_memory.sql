-- +goose Up
-- +goose StatementBegin
ALTER TABLE consolidated_memory ADD COLUMN metadata JSONB;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE consolidated_memory DROP COLUMN metadata;
-- +goose StatementEnd
