-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks_master ADD COLUMN IF NOT EXISTS _sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE shared_tasks_master ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE shared_tasks_master ADD COLUMN IF NOT EXISTS version INTEGER DEFAULT 1;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE shared_tasks_master DROP COLUMN IF EXISTS _sync_status;
ALTER TABLE shared_tasks_master DROP COLUMN IF EXISTS updated_at;
ALTER TABLE shared_tasks_master DROP COLUMN IF EXISTS version;
-- +goose StatementEnd
