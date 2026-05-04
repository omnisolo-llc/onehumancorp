-- 061_data_model_architecture.sql

-- Ensure pgvector is available for AI agent memories
CREATE EXTENSION IF NOT EXISTS vector;

-- 1. TENANTS
CREATE TABLE IF NOT EXISTS tenants (
    id UUID PRIMARY KEY,
    business_name TEXT NOT NULL,
    owner_email TEXT NOT NULL,
    subscription_tier TEXT NOT NULL DEFAULT 'free',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);

ALTER TABLE tenants ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tenants ON tenants;
CREATE POLICY tenant_isolation_tenants ON tenants
    USING (id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');

-- 2. PRODUCTS
CREATE TABLE IF NOT EXISTS products (
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    id UUID NOT NULL,
    type TEXT NOT NULL, -- physical, digital, service
    title TEXT NOT NULL,
    price_cents BIGINT NOT NULL DEFAULT 0,
    stock_level INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1,
    PRIMARY KEY (tenant_id, id)
);

ALTER TABLE products ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_products ON products;
CREATE POLICY tenant_isolation_products ON products
    USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');

-- 3. CUSTOMERS
CREATE TABLE IF NOT EXISTS customers (
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    id UUID NOT NULL,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1,
    PRIMARY KEY (tenant_id, id)
);

ALTER TABLE customers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customers ON customers;
CREATE POLICY tenant_isolation_customers ON customers
    USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');

-- 4. ORDER_BOOKINGS
CREATE TABLE IF NOT EXISTS order_bookings (
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    id UUID NOT NULL,
    customer_id UUID NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending', -- pending, paid, completed, cancelled
    total_amount_cents BIGINT NOT NULL DEFAULT 0,
    scheduled_for TIMESTAMPTZ, -- Null for instant orders
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1,
    PRIMARY KEY (tenant_id, id)
);

ALTER TABLE order_bookings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_order_bookings ON order_bookings;
CREATE POLICY tenant_isolation_order_bookings ON order_bookings
    USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');

-- 5. ORDER_ITEMS
CREATE TABLE IF NOT EXISTS order_items (
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    id UUID NOT NULL,
    order_id UUID NOT NULL,
    product_id UUID NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    unit_price_cents BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1,
    PRIMARY KEY (tenant_id, id)
);

ALTER TABLE order_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_order_items ON order_items;
CREATE POLICY tenant_isolation_order_items ON order_items
    USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');

-- 6. AI_AGENT_MEMORIES
CREATE TABLE IF NOT EXISTS ai_agent_memories (
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    id UUID NOT NULL,
    department TEXT NOT NULL, -- operations, marketing, finance...
    context_summary TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1,
    PRIMARY KEY (tenant_id, id)
);

ALTER TABLE ai_agent_memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ai_agent_memories ON ai_agent_memories;
CREATE POLICY tenant_isolation_ai_agent_memories ON ai_agent_memories
    USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
