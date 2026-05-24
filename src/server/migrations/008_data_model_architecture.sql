-- Migration 008: Data Model Architecture for OneHumanCorp

CREATE EXTENSION IF NOT EXISTS vector;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Core Schema Definitions
-- Ensure TENANT exists
CREATE TABLE IF NOT EXISTS tenants (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    industry TEXT DEFAULT '',
    tier TEXT DEFAULT 'free',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    version BIGINT DEFAULT 1
);

-- Ensure CUSTOMER exists
CREATE TABLE IF NOT EXISTS customers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    preferences JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Unified CATALOG_ITEM entity
CREATE TABLE IF NOT EXISTS catalog_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    item_type TEXT NOT NULL, -- Physical, Service, Digital, Subscription
    price DECIMAL DEFAULT 0,
    currency TEXT DEFAULT 'USD',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS item_variants (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    catalog_item_id TEXT REFERENCES catalog_items(id) ON DELETE CASCADE,
    sku TEXT,
    name TEXT,
    price_adjustment DECIMAL DEFAULT 0,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Event-Driven Inventory and Fulfillment
CREATE TABLE IF NOT EXISTS inventory_ledger (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    catalog_item_id TEXT REFERENCES catalog_items(id) ON DELETE CASCADE,
    variant_id TEXT REFERENCES item_variants(id) ON DELETE CASCADE,
    change_amount INT NOT NULL,
    reason TEXT NOT NULL,
    transaction_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- ORDER entity
CREATE TABLE IF NOT EXISTS orders (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT REFERENCES customers(id) ON DELETE CASCADE,
    total_amount DECIMAL DEFAULT 0,
    status TEXT DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- ORDER_LINE references ITEM_VARIANT
CREATE TABLE IF NOT EXISTS order_lines (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    order_id TEXT REFERENCES orders(id) ON DELETE CASCADE,
    variant_id TEXT REFERENCES item_variants(id) ON DELETE CASCADE,
    quantity INT DEFAULT 1,
    unit_price DECIMAL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS payments (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    order_id TEXT REFERENCES orders(id) ON DELETE CASCADE,
    amount DECIMAL NOT NULL,
    status TEXT DEFAULT 'pending',
    payment_method TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS fulfillments (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    order_id TEXT REFERENCES orders(id) ON DELETE CASCADE,
    status TEXT DEFAULT 'pending',
    tracking_number TEXT,
    provider TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS bookings (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    order_id TEXT REFERENCES orders(id) ON DELETE CASCADE,
    customer_id TEXT REFERENCES customers(id) ON DELETE CASCADE,
    variant_id TEXT REFERENCES item_variants(id) ON DELETE CASCADE,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ,
    status TEXT DEFAULT 'scheduled',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- AI Context
CREATE TABLE IF NOT EXISTS interactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT REFERENCES customers(id) ON DELETE CASCADE,
    channel TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS agent_actions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    agent_id TEXT NOT NULL,
    interaction_id TEXT REFERENCES interactions(id) ON DELETE CASCADE,
    action_type TEXT NOT NULL,
    payload JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enforce Strict Multi-Tenancy via PostgreSQL RLS
DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'tenants',
            'customers',
            'catalog_items',
            'item_variants',
            'inventory_ledger',
            'orders',
            'order_lines',
            'payments',
            'fulfillments',
            'bookings',
            'interactions',
            'agent_actions'
        ])
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN
            EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);

            pol_name := format('tenant_isolation_%s', t_name);
            IF NOT EXISTS (
                SELECT 1
                FROM pg_policies
                WHERE schemaname = current_schema()
                    AND tablename = t_name
                    AND policyname = pol_name
            ) THEN
                IF t_name = 'tenants' THEN
                    -- Tenants policy uses id instead of tenant_id
                    EXECUTE format(
                        'CREATE POLICY %I ON %I USING (id::text = current_setting(''app.current_tenant'', true))',
                        pol_name,
                        t_name
                    );
                ELSE
                    EXECUTE format(
                        'CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true))',
                        pol_name,
                        t_name
                    );
                END IF;
            END IF;
        END IF;
    END LOOP;
END
$$;
CREATE INDEX IF NOT EXISTS interactions_embedding_hnsw_idx ON interactions USING hnsw (embedding vector_cosine_ops);
