-- Add is_sold_out to products
ALTER TABLE products ADD COLUMN IF NOT EXISTS is_sold_out BOOLEAN DEFAULT FALSE;

-- Create product_variants table
CREATE TABLE IF NOT EXISTS product_variants (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    stock_level INT DEFAULT 0,
    price_override DECIMAL
);

-- Enable RLS on product_variants
ALTER TABLE product_variants ENABLE ROW LEVEL SECURITY;
ALTER TABLE product_variants FORCE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_product_variants' AND tablename = 'product_variants'
    ) THEN
        CREATE POLICY tenant_isolation_product_variants ON product_variants USING (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- Add deposit_paid to orders
ALTER TABLE orders ADD COLUMN IF NOT EXISTS deposit_paid DECIMAL DEFAULT 0;

-- Ensure bookings link to orders
ALTER TABLE bookings ADD COLUMN IF NOT EXISTS order_id TEXT REFERENCES orders(id) ON DELETE CASCADE;
ALTER TABLE bookings ADD COLUMN IF NOT EXISTS meeting_link TEXT;

-- Create ai_memory table using pgvector
CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS ai_memory (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    context_type TEXT NOT NULL,
    reference_id TEXT,
    embedding vector(1536),
    summary TEXT
);

-- Enable RLS on ai_memory
ALTER TABLE ai_memory ENABLE ROW LEVEL SECURITY;
ALTER TABLE ai_memory FORCE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_ai_memory' AND tablename = 'ai_memory'
    ) THEN
        CREATE POLICY tenant_isolation_ai_memory ON ai_memory USING (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;
