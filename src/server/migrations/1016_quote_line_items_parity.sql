-- 1016_quote_line_items_parity.sql
-- Ensure service_item_id and tenant_id exist on quote_line_items for quoting engine and isolation.

ALTER TABLE quote_line_items ADD COLUMN IF NOT EXISTS service_item_id UUID;
ALTER TABLE quote_line_items ADD COLUMN IF NOT EXISTS tenant_id TEXT;
