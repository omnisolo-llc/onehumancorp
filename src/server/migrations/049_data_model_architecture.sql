-- 049_data_model_architecture.sql
-- Foundational database migrations for OHC Data Model Architecture

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- TENANT table
CREATE TABLE IF NOT EXISTS tenants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    business_name TEXT NOT NULL,
    owner_email TEXT NOT NULL,
    subscription_tier TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- PRODUCT table
CREATE TABLE IF NOT EXISTS products (
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    type TEXT NOT NULL CHECK (type IN ('physical', 'digital', 'service')),
    title TEXT NOT NULL,
    price NUMERIC(15, 6) NOT NULL DEFAULT 0.0,
    stock_level INT NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    PRIMARY KEY (tenant_id, id)
);

-- CUSTOMER table
CREATE TABLE IF NOT EXISTS customers (
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT,
    PRIMARY KEY (tenant_id, id)
);

-- ORDER_BOOKING table
CREATE TABLE IF NOT EXISTS order_bookings (
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id UUID NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('pending', 'paid', 'completed', 'cancelled')),
    total_amount NUMERIC(15, 6) NOT NULL DEFAULT 0.0,
    scheduled_for TIMESTAMPTZ,
    PRIMARY KEY (tenant_id, id),
    FOREIGN KEY (tenant_id, customer_id) REFERENCES customers(tenant_id, id) ON DELETE CASCADE
);

-- ORDER_ITEM table
CREATE TABLE IF NOT EXISTS order_items (
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    order_id UUID NOT NULL,
    product_id UUID NOT NULL,
    quantity INT NOT NULL DEFAULT 1,
    unit_price NUMERIC(15, 6) NOT NULL DEFAULT 0.0,
    PRIMARY KEY (tenant_id, id),
    FOREIGN KEY (tenant_id, order_id) REFERENCES order_bookings(tenant_id, id) ON DELETE CASCADE,
    FOREIGN KEY (tenant_id, product_id) REFERENCES products(tenant_id, id) ON DELETE CASCADE
);

-- Enable Row Level Security (RLS) for all tenant-specific tables
ALTER TABLE products ENABLE ROW LEVEL SECURITY;
ALTER TABLE customers ENABLE ROW LEVEL SECURITY;
ALTER TABLE order_bookings ENABLE ROW LEVEL SECURITY;
ALTER TABLE order_items ENABLE ROW LEVEL SECURITY;

-- Create RLS Policies
-- Products
CREATE POLICY tenant_isolation_products ON products
    FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Customers
CREATE POLICY tenant_isolation_customers ON customers
    FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Order Bookings
CREATE POLICY tenant_isolation_order_bookings ON order_bookings
    FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Order Items
CREATE POLICY tenant_isolation_order_items ON order_items
    FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true));
