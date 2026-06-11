-- Add invisible split payment configuration to products
ALTER TABLE products ADD COLUMN IF NOT EXISTS split_partner_id TEXT;
ALTER TABLE products ADD COLUMN IF NOT EXISTS split_percentage DOUBLE PRECISION;

-- Add invisible split payment configuration to services
ALTER TABLE services ADD COLUMN IF NOT EXISTS split_partner_id TEXT;
ALTER TABLE services ADD COLUMN IF NOT EXISTS split_percentage DOUBLE PRECISION;

-- For the unified dual-write table we found
CREATE TABLE IF NOT EXISTS offerings (
    id TEXT PRIMARY KEY,
    tenant_id TEXT,
    type TEXT,
    title TEXT,
    description TEXT,
    price_cents BIGINT,
    metadata JSONB
);

ALTER TABLE offerings ADD COLUMN IF NOT EXISTS split_partner_id TEXT;
ALTER TABLE offerings ADD COLUMN IF NOT EXISTS split_percentage DOUBLE PRECISION;
