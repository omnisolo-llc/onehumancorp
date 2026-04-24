-- +goose Up
-- +goose StatementBegin
ALTER TABLE mcp_config_sync_log ADD COLUMN hlc_timestamp BIGINT;
ALTER TABLE hybrid_mcp_sync_queue ADD COLUMN hlc_timestamp BIGINT;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- SQLite has limited support for DROP COLUMN in older versions. Modern goose/sqlite usually handles it.
ALTER TABLE mcp_config_sync_log DROP COLUMN hlc_timestamp;
ALTER TABLE hybrid_mcp_sync_queue DROP COLUMN hlc_timestamp;
-- +goose StatementEnd
