-- +goose Up
CREATE TABLE IF NOT EXISTS tier_usage (
    tenant_id UUID PRIMARY KEY REFERENCES tenants(id) ON DELETE CASCADE,
    product_count INT NOT NULL DEFAULT 0,
    ai_actions_month INT NOT NULL DEFAULT 0,
    storage_bytes BIGINT NOT NULL DEFAULT 0,
    last_reset_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE tier_usage ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_tier_usage ON tier_usage
    USING (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_tier_usage ON tier_usage;
ALTER TABLE tier_usage DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS tier_usage;
