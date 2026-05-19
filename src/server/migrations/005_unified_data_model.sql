-- Migration 005: Unified Data Model with Strict Tenant Isolation

-- Enable pgvector and pgcrypto if not already enabled
CREATE EXTENSION IF NOT EXISTS vector;
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- 1. Tenants Table
CREATE TABLE IF NOT EXISTS tenants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    business_name TEXT NOT NULL,
    owner_email TEXT NOT NULL,
    subscription_tier TEXT NOT NULL DEFAULT 'free',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 2. Products Table
CREATE TABLE IF NOT EXISTS products (
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    type TEXT NOT NULL, -- physical, digital, service
    title TEXT NOT NULL,
    price_cents BIGINT NOT NULL DEFAULT 0,
    stock_level INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    version INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (tenant_id, id)
);

-- 3. Customers Table
CREATE TABLE IF NOT EXISTS customers (
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    version INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (tenant_id, id)
);

-- 4. Order Bookings Table
CREATE TABLE IF NOT EXISTS order_bookings (
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    customer_id UUID NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending', -- pending, paid, completed, cancelled
    total_amount_cents BIGINT NOT NULL DEFAULT 0,
    scheduled_for TIMESTAMPTZ, -- Null for instant orders
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    version INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (tenant_id, id)
);

-- 5. Order Items Table
CREATE TABLE IF NOT EXISTS order_items (
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL,
    product_id UUID NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    unit_price_cents BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (tenant_id, id)
);

-- 6. AI Agent Memory Table
CREATE TABLE IF NOT EXISTS agent_memories (
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    department TEXT NOT NULL, -- operations, marketing, finance...
    context_summary TEXT NOT NULL,
    embedding VECTOR(1536),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    version INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (tenant_id, id)
);

-- Enable RLS and Force RLS on all tables
ALTER TABLE tenants ENABLE ROW LEVEL SECURITY;
ALTER TABLE tenants FORCE ROW LEVEL SECURITY;

ALTER TABLE products ENABLE ROW LEVEL SECURITY;
ALTER TABLE products FORCE ROW LEVEL SECURITY;

ALTER TABLE customers ENABLE ROW LEVEL SECURITY;
ALTER TABLE customers FORCE ROW LEVEL SECURITY;

ALTER TABLE order_bookings ENABLE ROW LEVEL SECURITY;
ALTER TABLE order_bookings FORCE ROW LEVEL SECURITY;

ALTER TABLE order_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE order_items FORCE ROW LEVEL SECURITY;

ALTER TABLE agent_memories ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_memories FORCE ROW LEVEL SECURITY;

-- Create RLS Policies
CREATE POLICY tenant_isolation_tenants ON tenants USING (id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_products ON products USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_customers ON customers USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_order_bookings ON order_bookings USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_order_items ON order_items USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_agent_memories ON agent_memories USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Add foreign key constraints that include tenant_id for better isolation enforcement
ALTER TABLE order_bookings ADD CONSTRAINT fk_order_bookings_customer FOREIGN KEY (tenant_id, customer_id) REFERENCES customers(tenant_id, id);
ALTER TABLE order_items ADD CONSTRAINT fk_order_items_order FOREIGN KEY (tenant_id, order_id) REFERENCES order_bookings(tenant_id, id);
ALTER TABLE order_items ADD CONSTRAINT fk_order_items_product FOREIGN KEY (tenant_id, product_id) REFERENCES products(tenant_id, id);

-- Create indexes for performance
CREATE INDEX idx_products_tenant_id ON products(tenant_id);
CREATE INDEX idx_customers_tenant_id ON customers(tenant_id);
CREATE INDEX idx_order_bookings_tenant_id ON order_bookings(tenant_id);
CREATE INDEX idx_order_items_tenant_id ON order_items(tenant_id);
CREATE INDEX idx_agent_memories_tenant_id ON agent_memories(tenant_id);
