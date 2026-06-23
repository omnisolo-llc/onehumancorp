-- +goose Up
-- Enable missing RLS on specific tables

ALTER TABLE IF EXISTS telemetry_buffer ENABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS quote_line_items ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_telemetry_buffer ON telemetry_buffer USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE POLICY tenant_isolation_quote_line_items ON quote_line_items USING (
    quote_id IN (SELECT id FROM quotes WHERE tenant_id::text = current_setting('app.current_tenant', true))
) WITH CHECK (
    quote_id IN (SELECT id FROM quotes WHERE tenant_id::text = current_setting('app.current_tenant', true))
);

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_telemetry_buffer ON telemetry_buffer;
DROP POLICY IF EXISTS tenant_isolation_quote_line_items ON quote_line_items;
ALTER TABLE IF EXISTS telemetry_buffer DISABLE ROW LEVEL SECURITY;
ALTER TABLE IF EXISTS quote_line_items DISABLE ROW LEVEL SECURITY;
