CREATE TABLE IF NOT EXISTS tenant_quotas (
    tenant_id VARCHAR PRIMARY KEY,
    tier VARCHAR NOT NULL DEFAULT 'free',
    ai_action_usage BIGINT DEFAULT 0,
    storage_usage_bytes BIGINT DEFAULT 0
);
