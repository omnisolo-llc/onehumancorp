-- +goose Up
CREATE TABLE IF NOT EXISTS service_items (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    base_price_cents BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE service_items ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_service_items ON service_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Update QuoteLineItem to reference it if it's there
ALTER TABLE quote_line_items ADD COLUMN IF NOT EXISTS service_item_id UUID REFERENCES service_items(id) ON DELETE SET NULL;

-- +goose Down
ALTER TABLE quote_line_items DROP COLUMN IF EXISTS service_item_id;
DROP POLICY IF EXISTS tenant_isolation_service_items ON service_items;
DROP TABLE IF EXISTS service_items CASCADE;
