-- Migration 013: Business Data Model Evolution

-- Add owner_email to tenants
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS owner_email TEXT;

-- Create businesses table
CREATE TABLE IF NOT EXISTS businesses (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS on businesses
ALTER TABLE businesses ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_businesses ON businesses USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Add business_id to agent_memories
ALTER TABLE agent_memories ADD COLUMN IF NOT EXISTS business_id TEXT REFERENCES businesses(id) ON DELETE CASCADE;

-- Ensure RLS on agent_memories (redundant but safe per instruction)
ALTER TABLE agent_memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_memories ON agent_memories;
CREATE POLICY tenant_isolation_agent_memories ON agent_memories USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Ensure vector index exists for agent_memories
CREATE INDEX IF NOT EXISTS agent_memories_embedding_hnsw_idx ON agent_memories USING hnsw (embedding vector_cosine_ops);
