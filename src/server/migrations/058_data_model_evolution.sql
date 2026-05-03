-- 058_data_model_evolution.sql
-- Data Model Architecture: Robust Multi-Tenancy and AI Agent Memory

-- Rename organization_id to tenant_id on products
ALTER TABLE products RENAME COLUMN organization_id TO tenant_id;
DROP INDEX IF EXISTS idx_products_org;
CREATE INDEX idx_products_tenant ON products (tenant_id);

DROP POLICY IF EXISTS tenant_isolation_products ON products;
CREATE POLICY tenant_isolation_products ON products USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- Create orders table
CREATE TABLE IF NOT EXISTS orders (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    total_amount FLOAT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);
CREATE INDEX idx_orders_tenant ON orders (tenant_id);
ALTER TABLE orders ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_orders ON orders USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- Create order_lines table
CREATE TABLE IF NOT EXISTS order_lines (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    order_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);
CREATE INDEX idx_order_lines_tenant ON order_lines (tenant_id);
ALTER TABLE order_lines ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_order_lines ON order_lines USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- Create customers table
CREATE TABLE IF NOT EXISTS customers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);
CREATE INDEX idx_customers_tenant ON customers (tenant_id);
ALTER TABLE customers ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_customers ON customers USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- Create bookings table
CREATE TABLE IF NOT EXISTS bookings (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    status TEXT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);
CREATE INDEX idx_bookings_tenant ON bookings (tenant_id);
ALTER TABLE bookings ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_bookings ON bookings USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- autodream_memories updates
ALTER TABLE autodream_memories RENAME COLUMN organization_id TO tenant_id;
ALTER TABLE autodream_memories RENAME COLUMN task_id TO entity_id;
ALTER TABLE autodream_memories RENAME COLUMN source_type TO entity_type;

DROP INDEX IF EXISTS idx_autodream_org;
CREATE INDEX idx_autodream_tenant ON autodream_memories(tenant_id);

DROP POLICY IF EXISTS tenant_isolation_autodream_memories ON autodream_memories;
CREATE POLICY tenant_isolation_autodream_memories ON autodream_memories USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
