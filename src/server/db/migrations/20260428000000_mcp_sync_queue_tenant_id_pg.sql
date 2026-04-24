ALTER TABLE hybrid_mcp_sync_queue ADD COLUMN IF NOT EXISTS tenant_id VARCHAR(255) NOT NULL DEFAULT 'system';
