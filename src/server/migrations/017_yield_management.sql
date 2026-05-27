-- +goose Up
ALTER TABLE products ADD COLUMN IF NOT EXISTS auto_pricing_enabled BOOLEAN DEFAULT false;
ALTER TABLE products ADD COLUMN IF NOT EXISTS min_price DECIMAL;
ALTER TABLE products ADD COLUMN IF NOT EXISTS max_price DECIMAL;

CREATE TABLE IF NOT EXISTS yield_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    target_type TEXT NOT NULL,
    target_id TEXT NOT NULL,
    original_price DECIMAL,
    new_price DECIMAL,
    reason TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE yield_events ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_yield_events ON yield_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_yield_events ON yield_events;
DROP TABLE IF EXISTS yield_events;
ALTER TABLE products DROP COLUMN IF EXISTS auto_pricing_enabled;
ALTER TABLE products DROP COLUMN IF EXISTS min_price;
ALTER TABLE products DROP COLUMN IF EXISTS max_price;
