CREATE INDEX IF NOT EXISTS quotes_id_tenant_id_idx ON quotes ((id::text), tenant_id);
CREATE INDEX IF NOT EXISTS customers_id_tenant_id_idx ON customers ((id::text), tenant_id);
CREATE INDEX IF NOT EXISTS quote_line_items_quote_id_tenant_id_idx ON quote_line_items ((quote_id::text), tenant_id);
