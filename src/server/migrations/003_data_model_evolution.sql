-- Migration: 003_data_model_evolution.sql
-- Foundational SQL schemas for Tenant, Business, and AgentMemory models
-- including RLS and vector indexing.

-- Ensure vector extension exists
CREATE EXTENSION IF NOT EXISTS vector;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Tenant is already present in 001_initial.sql, but we need to ensure owner_email and tier exist.
-- To match the struct strictly and safely modify, we just add missing columns.
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS owner_email TEXT DEFAULT '';
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS tier TEXT DEFAULT 'free';

-- Business Table
CREATE TABLE IF NOT EXISTS businesses (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Apply RLS to Business
ALTER TABLE businesses ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_businesses ON businesses USING (tenant_id::text = current_setting('app.current_tenant', true));

-- AgentMemory Table
CREATE TABLE IF NOT EXISTS agent_memory (
    id TEXT PRIMARY KEY,
    business_id TEXT NOT NULL REFERENCES businesses(id) ON DELETE CASCADE,
    department TEXT NOT NULL,
    embeddings VECTOR(1536),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- We need a way to link agent_memory to a tenant for RLS, or we use a join/function.
-- The safest way for standard RLS is adding a tenant_id column.
ALTER TABLE agent_memory ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE;

-- Apply RLS to AgentMemory
ALTER TABLE agent_memory ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_memory ON agent_memory USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Vector Indexing for AgentMemory
CREATE INDEX IF NOT EXISTS agent_memory_embeddings_idx ON agent_memory USING ivfflat (embeddings vector_cosine_ops) WITH (lists = 100);
