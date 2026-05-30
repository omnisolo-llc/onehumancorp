CREATE TABLE IF NOT EXISTS inventory_forecasts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    item_id TEXT NOT NULL,
    predicted_stockout_date TIMESTAMPTZ,
    recommended_restock_qty INT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_inventory_forecasts_tenant_id ON inventory_forecasts(tenant_id);
CREATE INDEX IF NOT EXISTS idx_inventory_forecasts_item_id ON inventory_forecasts(item_id);

ALTER TABLE inventory_forecasts ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_inventory_forecasts ON inventory_forecasts USING (tenant_id::text = current_setting('app.current_tenant', true));
