-- Migration 052: Predictive Inventory Engine

CREATE TABLE IF NOT EXISTS supplier_orders (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    status TEXT DEFAULT 'DRAFT',
    quantity INT DEFAULT 0,
    draft_content TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE supplier_orders ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_supplier_orders ON supplier_orders USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS stock_forecasts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    daily_velocity DECIMAL DEFAULT 0,
    days_until_stockout DECIMAL DEFAULT 0,
