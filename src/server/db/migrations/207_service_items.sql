CREATE TABLE IF NOT EXISTS service_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    base_price BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE service_items ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_service_items ON service_items FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE quote_line_items ADD COLUMN IF NOT EXISTS service_item_id TEXT REFERENCES service_items(id) ON DELETE SET NULL;
