-- +goose Up
-- +goose StatementBegin
ALTER TABLE mcp_config_sync_log ADD COLUMN IF NOT EXISTS hlc_timestamp BIGINT;
ALTER TABLE mcp_audit_sync_log ADD COLUMN IF NOT EXISTS hlc_timestamp BIGINT;
ALTER TABLE hybrid_mcp_sync_queue ADD COLUMN IF NOT EXISTS hlc_timestamp BIGINT;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE mcp_config_sync_log DROP COLUMN IF EXISTS hlc_timestamp;
ALTER TABLE mcp_audit_sync_log DROP COLUMN IF EXISTS hlc_timestamp;
ALTER TABLE hybrid_mcp_sync_queue DROP COLUMN IF EXISTS hlc_timestamp;
-- +goose StatementEnd
