-- +goose Up
-- +goose StatementBegin
ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS synced_to_cloud BOOLEAN DEFAULT false;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE autodream_memories DROP COLUMN IF NOT EXISTS synced_to_cloud;
-- +goose StatementEnd
