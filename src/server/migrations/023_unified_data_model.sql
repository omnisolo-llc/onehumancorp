-- +goose Up
-- Migration 023: Unified Data Model Evolution

-- Add soft delete, versioning, and sync properties to core tables
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS is_deleted BOOLEAN DEFAULT false;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS last_synced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS version_hash TEXT;

ALTER TABLE users ADD COLUMN IF NOT EXISTS is_deleted BOOLEAN DEFAULT false;
ALTER TABLE users ADD COLUMN IF NOT EXISTS last_synced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE users ADD COLUMN IF NOT EXISTS version_hash TEXT;

ALTER TABLE products ADD COLUMN IF NOT EXISTS base_price DECIMAL DEFAULT 0;
ALTER TABLE products ADD COLUMN IF NOT EXISTS is_active BOOLEAN DEFAULT true;
ALTER TABLE products ADD COLUMN IF NOT EXISTS is_deleted BOOLEAN DEFAULT false;
ALTER TABLE products ADD COLUMN IF NOT EXISTS last_synced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE products ADD COLUMN IF NOT EXISTS version_hash TEXT;

ALTER TABLE orders ADD COLUMN IF NOT EXISTS payment_status TEXT DEFAULT 'pending';
ALTER TABLE orders ADD COLUMN IF NOT EXISTS is_deleted BOOLEAN DEFAULT false;
ALTER TABLE orders ADD COLUMN IF NOT EXISTS last_synced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE orders ADD COLUMN IF NOT EXISTS version_hash TEXT;

ALTER TABLE customers ADD COLUMN IF NOT EXISTS user_id TEXT;
ALTER TABLE customers ADD COLUMN IF NOT EXISTS contact_info TEXT;
ALTER TABLE customers ADD COLUMN IF NOT EXISTS ltv DECIMAL DEFAULT 0;
ALTER TABLE customers ADD COLUMN IF NOT EXISTS is_deleted BOOLEAN DEFAULT false;
ALTER TABLE customers ADD COLUMN IF NOT EXISTS last_synced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE customers ADD COLUMN IF NOT EXISTS version_hash TEXT;

ALTER TABLE bookings ADD COLUMN IF NOT EXISTS service_id TEXT;
ALTER TABLE bookings ADD COLUMN IF NOT EXISTS is_deleted BOOLEAN DEFAULT false;
ALTER TABLE bookings ADD COLUMN IF NOT EXISTS last_synced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE bookings ADD COLUMN IF NOT EXISTS version_hash TEXT;

-- Create ProductVariant table
CREATE TABLE IF NOT EXISTS product_variants (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    attributes JSONB DEFAULT '{}',
    price_adjustment DECIMAL DEFAULT 0,
    sku TEXT,
    is_deleted BOOLEAN DEFAULT false,
    last_synced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    version_hash TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Create Inventory table
CREATE TABLE IF NOT EXISTS inventory (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    variant_id TEXT NOT NULL REFERENCES product_variants(id) ON DELETE CASCADE,
    quantity INT DEFAULT 0,
    location_id TEXT,
    reserved_quantity INT DEFAULT 0,
    is_deleted BOOLEAN DEFAULT false,
    last_synced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    version_hash TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Ensure OrderItems has necessary columns
ALTER TABLE order_items ADD COLUMN IF NOT EXISTS variant_id TEXT REFERENCES product_variants(id) ON DELETE CASCADE;
ALTER TABLE order_items ADD COLUMN IF NOT EXISTS price_at_time DECIMAL DEFAULT 0;
ALTER TABLE order_items ADD COLUMN IF NOT EXISTS is_deleted BOOLEAN DEFAULT false;
ALTER TABLE order_items ADD COLUMN IF NOT EXISTS last_synced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE order_items ADD COLUMN IF NOT EXISTS version_hash TEXT;

-- Create Message table
CREATE TABLE IF NOT EXISTS messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    channel TEXT NOT NULL,
    direction TEXT NOT NULL,
    content TEXT NOT NULL,
    ai_handled BOOLEAN DEFAULT false,
    is_deleted BOOLEAN DEFAULT false,
    last_synced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    version_hash TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Create AgentActivity table
CREATE TABLE IF NOT EXISTS agent_activities (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    agent_department TEXT NOT NULL,
    action_type TEXT NOT NULL,
    status TEXT NOT NULL,
    details TEXT,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    is_deleted BOOLEAN DEFAULT false,
    last_synced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    version_hash TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Dashboard Summary Materialized View
CREATE MATERIALIZED VIEW IF NOT EXISTS dashboard_summary AS
SELECT
    t.id AS tenant_id,
    COUNT(DISTINCT o.id) AS total_orders,
    SUM(o.total_amount) AS total_revenue,
    COUNT(DISTINCT c.id) AS total_customers,
    COUNT(DISTINCT b.id) AS total_bookings,
    COUNT(DISTINCT a.id) AS active_ai_tasks,
    CURRENT_TIMESTAMP AS calculated_at
FROM tenants t
LEFT JOIN orders o ON t.id = o.tenant_id AND o.is_deleted = false
LEFT JOIN customers c ON t.id = c.tenant_id AND c.is_deleted = false
LEFT JOIN bookings b ON t.id = b.tenant_id AND b.is_deleted = false
LEFT JOIN agent_activities a ON t.id = a.tenant_id AND a.is_deleted = false AND a.status = 'PENDING'
GROUP BY t.id;

CREATE UNIQUE INDEX IF NOT EXISTS idx_dashboard_summary_tenant_id ON dashboard_summary(tenant_id);

-- Enforce Multi-Tenancy
ALTER TABLE product_variants ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_product_variants ON product_variants;
CREATE POLICY tenant_isolation_product_variants ON product_variants USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE inventory ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inventory ON inventory;
CREATE POLICY tenant_isolation_inventory ON inventory USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_messages ON messages;
CREATE POLICY tenant_isolation_messages ON messages USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agent_activities ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_activities ON agent_activities;
CREATE POLICY tenant_isolation_agent_activities ON agent_activities USING (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
-- Migration 023 Down

DROP MATERIALIZED VIEW IF NOT EXISTS dashboard_summary;

DROP POLICY IF EXISTS tenant_isolation_agent_activities ON agent_activities;
ALTER TABLE agent_activities DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS agent_activities CASCADE;

DROP POLICY IF EXISTS tenant_isolation_messages ON messages;
ALTER TABLE messages DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS messages CASCADE;

ALTER TABLE order_items DROP COLUMN IF EXISTS variant_id;
ALTER TABLE order_items DROP COLUMN IF EXISTS price_at_time;
ALTER TABLE order_items DROP COLUMN IF EXISTS is_deleted;
ALTER TABLE order_items DROP COLUMN IF EXISTS last_synced_at;
ALTER TABLE order_items DROP COLUMN IF EXISTS version_hash;

DROP POLICY IF EXISTS tenant_isolation_inventory ON inventory;
ALTER TABLE inventory DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS inventory CASCADE;

DROP POLICY IF EXISTS tenant_isolation_product_variants ON product_variants;
ALTER TABLE product_variants DISABLE ROW LEVEL SECURITY;
DROP TABLE IF EXISTS product_variants CASCADE;

ALTER TABLE bookings DROP COLUMN IF EXISTS service_id;
ALTER TABLE bookings DROP COLUMN IF EXISTS is_deleted;
ALTER TABLE bookings DROP COLUMN IF EXISTS last_synced_at;
ALTER TABLE bookings DROP COLUMN IF EXISTS version_hash;

ALTER TABLE customers DROP COLUMN IF EXISTS user_id;
ALTER TABLE customers DROP COLUMN IF EXISTS contact_info;
ALTER TABLE customers DROP COLUMN IF EXISTS ltv;
ALTER TABLE customers DROP COLUMN IF EXISTS is_deleted;
ALTER TABLE customers DROP COLUMN IF EXISTS last_synced_at;
ALTER TABLE customers DROP COLUMN IF EXISTS version_hash;

ALTER TABLE orders DROP COLUMN IF EXISTS payment_status;
ALTER TABLE orders DROP COLUMN IF EXISTS is_deleted;
ALTER TABLE orders DROP COLUMN IF EXISTS last_synced_at;
ALTER TABLE orders DROP COLUMN IF EXISTS version_hash;

ALTER TABLE products DROP COLUMN IF EXISTS base_price;
ALTER TABLE products DROP COLUMN IF EXISTS is_active;
ALTER TABLE products DROP COLUMN IF EXISTS is_deleted;
ALTER TABLE products DROP COLUMN IF EXISTS last_synced_at;
ALTER TABLE products DROP COLUMN IF EXISTS version_hash;

ALTER TABLE users DROP COLUMN IF EXISTS is_deleted;
ALTER TABLE users DROP COLUMN IF EXISTS last_synced_at;
ALTER TABLE users DROP COLUMN IF EXISTS version_hash;

ALTER TABLE tenants DROP COLUMN IF EXISTS is_deleted;
ALTER TABLE tenants DROP COLUMN IF EXISTS last_synced_at;
ALTER TABLE tenants DROP COLUMN IF EXISTS version_hash;
