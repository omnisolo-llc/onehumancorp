-- Migration 229: Quotes and Builder sites column parity

ALTER TABLE quotes ADD COLUMN IF NOT EXISTS valid_until TIMESTAMPTZ;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS proposed_slot_id TEXT;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS service_id TEXT;

ALTER TABLE builder_sites ADD COLUMN IF NOT EXISTS published_at TIMESTAMPTZ;

ALTER TABLE quote_line_items ADD COLUMN IF NOT EXISTS service_item_id UUID;
ALTER TABLE quote_line_items ADD COLUMN IF NOT EXISTS tenant_id TEXT;
