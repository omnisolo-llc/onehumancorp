-- Consolidated Initial Schema for OHC
-- This file contains all table definitions, indexes, and RLS policies.

CREATE EXTENSION IF NOT EXISTS vector;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 1. Core Tables
CREATE TABLE IF NOT EXISTS tenants (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    industry TEXT DEFAULT '',
    tier TEXT DEFAULT 'free',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    version BIGINT DEFAULT 1
);

CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL DEFAULT '',
    roles TEXT[] DEFAULT '{}',
    active BOOLEAN DEFAULT TRUE,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    oidc_subject TEXT UNIQUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS roles (
    id TEXT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    permissions TEXT[] DEFAULT '{}',
    tenant_id TEXT DEFAULT 'system',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS revoked_tokens (
    jti TEXT PRIMARY KEY,
    tenant_id TEXT DEFAULT 'system',
    expires_at TIMESTAMPTZ NOT NULL
);

-- 2. Agentic Tables
CREATE TABLE IF NOT EXISTS agents (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    role TEXT NOT NULL,
    status TEXT DEFAULT 'IDLE',
    provider_type TEXT,
    region TEXT,
    registered_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS agent_missions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    status TEXT NOT NULL,
    payload TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    synced_to_cloud BOOLEAN DEFAULT false
);

CREATE TABLE IF NOT EXISTS agent_status (
    agent_id TEXT PRIMARY KEY REFERENCES agents(id) ON DELETE CASCADE,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    role TEXT NOT NULL,
    status TEXT NOT NULL,
    last_heartbeat TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    parent_task_id TEXT REFERENCES tasks(id),
    title TEXT NOT NULL,
    description TEXT,
    status TEXT DEFAULT 'PENDING',
    assigned_agent_id TEXT REFERENCES agents(id),
    payload TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id TEXT REFERENCES tasks(id) ON DELETE CASCADE,
    depends_on_task_id TEXT REFERENCES tasks(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, depends_on_task_id)
);

-- 3. Commerce Tables
CREATE TABLE IF NOT EXISTS products (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    type TEXT, -- 'physical', 'digital', 'booking'
    price DECIMAL DEFAULT 0,
    price_cents BIGINT DEFAULT 0,
    currency TEXT DEFAULT 'USD',
    inventory_count INT DEFAULT 0,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

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

CREATE TABLE IF NOT EXISTS orders (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT REFERENCES customers(id) ON DELETE CASCADE,
    total_amount DECIMAL DEFAULT 0,
    status TEXT DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS order_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    order_id TEXT REFERENCES orders(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    quantity INT DEFAULT 1,
    price DECIMAL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS bookings (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT REFERENCES customers(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ,
    status TEXT DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 4. Knowledge & Memory
CREATE TABLE IF NOT EXISTS agent_memories (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    department TEXT,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    interaction_data JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS knowledge_embeddings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    agent_id TEXT,
    task_id TEXT,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    source_type TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 5. RLS Policies
ALTER TABLE tenants ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_tenants ON tenants USING (id::text = current_setting('app.current_tenant', true));

ALTER TABLE users ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_users ON users USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agents ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agents ON agents USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_tasks ON tasks USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE products ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_products ON products USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE orders ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_orders ON orders USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE customers ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_customers ON customers USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE bookings ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_bookings ON bookings USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agent_memories ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_memories ON agent_memories USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE knowledge_embeddings ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_knowledge_embeddings ON knowledge_embeddings USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Add other tables as needed...
-- For brevity and correctness, I will use the previous content but cleaned up.
ALTER TABLE roles ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_roles ON roles USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE revoked_tokens ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_revoked_tokens ON revoked_tokens USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agent_missions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_missions ON agent_missions USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE agent_status ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_status ON agent_status USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE order_items ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_order_items ON order_items USING (tenant_id::text = current_setting('app.current_tenant', true));
CREATE INDEX IF NOT EXISTS agent_memories_embedding_hnsw_idx ON agent_memories USING hnsw (embedding vector_cosine_ops);
CREATE INDEX IF NOT EXISTS knowledge_embeddings_embedding_hnsw_idx ON knowledge_embeddings USING hnsw (embedding vector_cosine_ops);
