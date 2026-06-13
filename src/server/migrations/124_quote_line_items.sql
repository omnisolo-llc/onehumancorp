CREATE TABLE IF NOT EXISTS quote_line_items (
    id TEXT PRIMARY KEY,
    quote_id TEXT REFERENCES quotes(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    unit_price_cents BIGINT NOT NULL,
    quantity INT NOT NULL DEFAULT 1,
    is_optional BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE quotes ADD COLUMN IF NOT EXISTS customer_id TEXT REFERENCES customers(id) ON DELETE SET NULL;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS valid_until TIMESTAMPTZ;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ DEFAULT NOW();
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();

ALTER TABLE quote_line_items ADD COLUMN IF NOT EXISTS tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE;

ALTER TABLE quote_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_quote_line_items ON quote_line_items;
CREATE POLICY tenant_isolation_quote_line_items ON quote_line_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
