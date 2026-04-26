-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS mcp_config_sync_log (
    tenant_id VARCHAR(255) NOT NULL,
    agent_id VARCHAR(255) NOT NULL,
    key VARCHAR(255) NOT NULL,
    value TEXT NOT NULL,
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_mcp_config_sync_log_lookup ON mcp_config_sync_log (tenant_id, key, created_at DESC);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF NOT EXISTS mcp_config_sync_log;
-- +goose StatementEnd
