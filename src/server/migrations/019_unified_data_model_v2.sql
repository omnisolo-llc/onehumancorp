-- Migration 019: Unified Data Model Foundation (v2)

CREATE EXTENSION IF NOT EXISTS vector;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Core Table Adjustments (Assuming they mostly exist from 001, but ensuring compliance with CUJ)

-- 1. Tenant
-- Note: Re-creating tables and adding columns without renaming `id` because there are too many foreign key dependencies on `tenants(id)`.
-- We will just add `tenant_id` to `tenants` and sync it.
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS tenant_id TEXT;
UPDATE tenants SET tenant_id = id WHERE tenant_id IS NULL;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS business_name TEXT;
UPDATE tenants SET business_name = name WHERE business_name IS NULL;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS owner_id TEXT;
DROP POLICY IF EXISTS tenant_isolation_tenants ON tenants;
CREATE POLICY tenant_isolation_tenants ON tenants USING (id::text = current_setting('app.current_tenant', true));

-- 2. Customer
CREATE TABLE IF NOT EXISTS customers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    email TEXT,
    phone TEXT,
    name TEXT,
    preferences JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE customers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customers ON customers;
CREATE POLICY tenant_isolation_customers ON customers USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 3. Product
CREATE TABLE IF NOT EXISTS products (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    type TEXT,
    title TEXT,
    price DECIMAL,
    inventory_count INT
);
ALTER TABLE products ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_products ON products;
CREATE POLICY tenant_isolation_products ON products USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 4. Order
CREATE TABLE IF NOT EXISTS orders (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT REFERENCES customers(id) ON DELETE CASCADE,
    total_amount DECIMAL,
    status TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE orders ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_orders ON orders;
CREATE POLICY tenant_isolation_orders ON orders USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 5. Booking
CREATE TABLE IF NOT EXISTS bookings (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT REFERENCES customers(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ,
    status TEXT
);
ALTER TABLE bookings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_bookings ON bookings;
CREATE POLICY tenant_isolation_bookings ON bookings USING (tenant_id::text = current_setting('app.current_tenant', true));

-- 6. AgentMemory
CREATE TABLE IF NOT EXISTS agent_memories (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    department TEXT,
    context_embedding VECTOR(1536),
    interaction_data JSONB DEFAULT '{}'
);
ALTER TABLE agent_memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_memories ON agent_memories;
CREATE POLICY tenant_isolation_agent_memories ON agent_memories USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix context_embedding on existing tables if needed
ALTER TABLE agent_memories ADD COLUMN IF NOT EXISTS context_embedding VECTOR(1536);
-- Migrate data if embedding existed
DO $$
BEGIN
  IF EXISTS(SELECT 1 FROM information_schema.columns WHERE table_name='agent_memories' and column_name='embedding') THEN
    UPDATE agent_memories SET context_embedding = embedding WHERE context_embedding IS NULL;
    ALTER TABLE agent_memories DROP COLUMN embedding;
  END IF;
END $$;

DROP INDEX IF EXISTS agent_memories_embedding_hnsw_idx;
CREATE INDEX IF NOT EXISTS agent_memories_context_embedding_hnsw_idx ON agent_memories USING hnsw (context_embedding vector_cosine_ops);
