CREATE TABLE IF NOT EXISTS usage_idempotency_keys (
    key TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (key, tenant_id)
);

CREATE INDEX IF NOT EXISTS idx_usage_idempotency_keys_tenant_id ON usage_idempotency_keys(tenant_id);
