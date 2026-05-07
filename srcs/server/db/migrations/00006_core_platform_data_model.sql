-- +goose Up
-- Create core platform data model tables

CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS tenant (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR NOT NULL,
    domain VARCHAR NOT NULL,
    tier VARCHAR NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS customer (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenant(id) ON DELETE CASCADE,
    email VARCHAR NOT NULL,
    phone VARCHAR,
    preferences JSONB DEFAULT '{}',
    last_active TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS catalog_item (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenant(id) ON DELETE CASCADE,
    title VARCHAR NOT NULL,
    description VARCHAR,
    item_type VARCHAR NOT NULL, -- "product | service | digital | subscription"
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS item_variant (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenant(id) ON DELETE CASCADE,
    catalog_item_id UUID NOT NULL REFERENCES catalog_item(id) ON DELETE CASCADE,
    sku VARCHAR NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    inventory_count INT NOT NULL DEFAULT 0,
    attributes JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "order" (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenant(id) ON DELETE CASCADE,
    customer_id UUID NOT NULL REFERENCES customer(id) ON DELETE CASCADE,
    status VARCHAR NOT NULL, -- "draft | pending_payment | confirmed | fulfilled | cancelled"
    total_amount DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS order_line_item (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenant(id) ON DELETE CASCADE,
    order_id UUID NOT NULL REFERENCES "order"(id) ON DELETE CASCADE,
    variant_id UUID NOT NULL REFERENCES item_variant(id) ON DELETE CASCADE,
    quantity INT NOT NULL,
    unit_price DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS agent_memory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenant(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES customer(id) ON DELETE SET NULL,
    department VARCHAR NOT NULL,
    embedding vector(1536),
    raw_context JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS
ALTER TABLE tenant ENABLE ROW LEVEL SECURITY;
ALTER TABLE customer ENABLE ROW LEVEL SECURITY;
ALTER TABLE catalog_item ENABLE ROW LEVEL SECURITY;
ALTER TABLE item_variant ENABLE ROW LEVEL SECURITY;
ALTER TABLE "order" ENABLE ROW LEVEL SECURITY;
ALTER TABLE order_line_item ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_memory ENABLE ROW LEVEL SECURITY;

-- Tenant Isolation Policies
CREATE POLICY tenant_isolation_tenant ON tenant
    USING (id = current_setting('app.current_tenant', true)::uuid);

CREATE POLICY tenant_isolation_customer ON customer
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE POLICY tenant_isolation_catalog_item ON catalog_item
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE POLICY tenant_isolation_item_variant ON item_variant
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE POLICY tenant_isolation_order ON "order"
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE POLICY tenant_isolation_order_line_item ON order_line_item
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE POLICY tenant_isolation_agent_memory ON agent_memory
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_agent_memory ON agent_memory;
DROP POLICY IF EXISTS tenant_isolation_order_line_item ON order_line_item;
DROP POLICY IF EXISTS tenant_isolation_order ON "order";
DROP POLICY IF EXISTS tenant_isolation_item_variant ON item_variant;
DROP POLICY IF EXISTS tenant_isolation_catalog_item ON catalog_item;
DROP POLICY IF EXISTS tenant_isolation_customer ON customer;
DROP POLICY IF EXISTS tenant_isolation_tenant ON tenant;

ALTER TABLE agent_memory DISABLE ROW LEVEL SECURITY;
ALTER TABLE order_line_item DISABLE ROW LEVEL SECURITY;
ALTER TABLE "order" DISABLE ROW LEVEL SECURITY;
ALTER TABLE item_variant DISABLE ROW LEVEL SECURITY;
ALTER TABLE catalog_item DISABLE ROW LEVEL SECURITY;
ALTER TABLE customer DISABLE ROW LEVEL SECURITY;
ALTER TABLE tenant DISABLE ROW LEVEL SECURITY;

DROP TABLE IF EXISTS agent_memory;
DROP TABLE IF EXISTS order_line_item;
DROP TABLE IF EXISTS "order";
DROP TABLE IF EXISTS item_variant;
DROP TABLE IF EXISTS catalog_item;
DROP TABLE IF EXISTS customer;
DROP TABLE IF EXISTS tenant;
