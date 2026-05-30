-- +goose Up
-- Migration 022: Core Platform Data Model Architecture

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS vector;

-- 1. tenant
CREATE TABLE IF NOT EXISTS tenant (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    domain TEXT,
    tier TEXT DEFAULT 'free',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 2. customer
CREATE TABLE IF NOT EXISTS customer (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenant(id) ON DELETE CASCADE,
    email TEXT,
    phone TEXT,
    preferences JSONB DEFAULT '{}',
    last_active TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 3. catalog_item
CREATE TABLE IF NOT EXISTS catalog_item (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenant(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    item_type TEXT NOT NULL, -- "product | service | digital | subscription"
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 4. item_variant
CREATE TABLE IF NOT EXISTS item_variant (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenant(id) ON DELETE CASCADE,
    catalog_item_id UUID NOT NULL REFERENCES catalog_item(id) ON DELETE CASCADE,
    sku TEXT,
    price DECIMAL(12,2) NOT NULL,
    inventory_count INT DEFAULT 0,
    attributes JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 5. order
CREATE TABLE IF NOT EXISTS "order" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenant(id) ON DELETE CASCADE,
    customer_id UUID NOT NULL REFERENCES customer(id) ON DELETE CASCADE,
    status TEXT DEFAULT 'draft', -- "draft | pending_payment | confirmed | fulfilled | cancelled"
    total_amount DECIMAL(12,2) DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 6. order_line_item
CREATE TABLE IF NOT EXISTS order_line_item (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenant(id) ON DELETE CASCADE,
    order_id UUID NOT NULL REFERENCES "order"(id) ON DELETE CASCADE,
    variant_id UUID NOT NULL REFERENCES item_variant(id) ON DELETE CASCADE,
    quantity INT DEFAULT 1,
    unit_price DECIMAL(12,2) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 7. agent_memory
CREATE TABLE IF NOT EXISTS agent_memory (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenant(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES customer(id) ON DELETE SET NULL,
    department TEXT NOT NULL, -- "sales | ops | support | etc"
    embedding VECTOR(1536),
    raw_context JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- RLS Enforcement
DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'tenant',
            'customer',
            'catalog_item',
            'item_variant',
            'order',
            'order_line_item',
            'agent_memory'
        ])
    LOOP
        EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);

        pol_name := 'tenant_isolation_policy';

        IF t_name = 'tenant' THEN
            EXECUTE format(
                'CREATE POLICY %I ON %I USING (id = NULLIF(current_setting(''app.current_tenant'', true), '''')::uuid)',
                pol_name,
                t_name
            );
        ELSE
            EXECUTE format(
                'CREATE POLICY %I ON %I USING (tenant_id = NULLIF(current_setting(''app.current_tenant'', true), '''')::uuid)',
                pol_name,
                t_name
            );
        END IF;
    END LOOP;
END
$$;

-- Indexes
CREATE INDEX IF NOT EXISTS agent_memory_embedding_idx ON agent_memory USING hnsw (embedding vector_cosine_ops);
CREATE INDEX IF NOT EXISTS idx_customer_tenant ON customer(tenant_id);
CREATE INDEX IF NOT EXISTS idx_catalog_item_tenant ON catalog_item(tenant_id);
CREATE INDEX IF NOT EXISTS idx_item_variant_tenant ON item_variant(tenant_id);
CREATE INDEX IF NOT EXISTS idx_order_tenant ON "order"(tenant_id);
CREATE INDEX IF NOT EXISTS idx_order_line_item_tenant ON order_line_item(tenant_id);
CREATE INDEX IF NOT EXISTS idx_order_customer ON "order"(customer_id);

-- +goose Down
DROP TABLE IF EXISTS agent_memory CASCADE;
DROP TABLE IF EXISTS order_line_item CASCADE;
DROP TABLE IF EXISTS "order" CASCADE;
DROP TABLE IF EXISTS item_variant CASCADE;
DROP TABLE IF EXISTS catalog_item CASCADE;
DROP TABLE IF EXISTS customer CASCADE;
DROP TABLE IF EXISTS tenant CASCADE;
