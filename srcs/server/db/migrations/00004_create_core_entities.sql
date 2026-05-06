-- +goose Up
-- Enable Row Level Security (RLS) on all tables created here

-- 1. Modify tenants to add RLS
ALTER TABLE tenants ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tenants ON tenants;
CREATE POLICY tenant_isolation_tenants ON tenants
    FOR ALL
    USING (id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- 2. Create products table
CREATE TABLE IF NOT EXISTS products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    type VARCHAR NOT NULL CHECK (type IN ('physical', 'digital', 'food')),
    inventory_count INT DEFAULT 0,
    is_sold_out BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE products ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_products ON products
    FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- 3. Create customers table
CREATE TABLE IF NOT EXISTS customers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    email VARCHAR,
    preferences JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE customers ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_customers ON customers
    FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- 4. Create orders table
CREATE TABLE IF NOT EXISTS orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES customers(id) ON DELETE CASCADE,
    status VARCHAR NOT NULL CHECK (status IN ('pending', 'paid', 'fulfilled')),
    total_amount DECIMAL(10, 2) DEFAULT 0.00,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE orders ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_orders ON orders
    FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_orders ON orders;
DROP TABLE IF EXISTS orders;

DROP POLICY IF EXISTS tenant_isolation_customers ON customers;
DROP TABLE IF EXISTS customers;

DROP POLICY IF EXISTS tenant_isolation_products ON products;
DROP TABLE IF EXISTS products;

DROP POLICY IF EXISTS tenant_isolation_tenants ON tenants;
ALTER TABLE tenants DISABLE ROW LEVEL SECURITY;
