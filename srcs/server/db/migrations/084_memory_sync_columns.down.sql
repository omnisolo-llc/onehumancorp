-- +goose Down
ALTER TABLE consolidated_memory DROP COLUMN IF EXISTS updated_at;
ALTER TABLE consolidated_memory DROP COLUMN IF EXISTS version;
