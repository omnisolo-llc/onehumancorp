-- +goose Up
-- +goose StatementBegin
ALTER TABLE autodream_memories ADD COLUMN synced BOOLEAN DEFAULT false;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE autodream_memories DROP COLUMN synced;
-- +goose StatementEnd
