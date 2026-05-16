-- Migration 003: Agent Departments

-- Enforce pgvector exists
CREATE EXTENSION IF NOT EXISTS vector;

-- Agent Departments table
CREATE TABLE agent_departments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    department_type VARCHAR NOT NULL, -- e.g., 'customer_success', 'operations', 'marketing', 'sales', 'finance', 'legal', 'business_advisory'
    config JSONB NOT NULL DEFAULT '{}',
    is_active BOOLEAN NOT NULL DEFAULT true,
    budget_remaining INT NOT NULL DEFAULT 100,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, department_type)
);

-- Agent Interactions Log table
CREATE TABLE agent_interaction_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    department_id UUID NOT NULL REFERENCES agent_departments(id) ON DELETE CASCADE,
    action_type VARCHAR NOT NULL,
    description TEXT NOT NULL,
    confidence_score FLOAT NOT NULL,
    status VARCHAR NOT NULL, -- e.g., 'pending_approval', 'approved', 'executed', 'failed'
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Consolidated Memory table (using pgvector)
CREATE TABLE agent_consolidated_memory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    department_id UUID NOT NULL REFERENCES agent_departments(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    embedding vector(1536) NOT NULL,
    source_type VARCHAR NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Row-Level Security (RLS) policies
ALTER TABLE agent_departments ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_interaction_logs ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_consolidated_memory ENABLE ROW LEVEL SECURITY;

-- Create policies based on tenant_id
CREATE POLICY agent_departments_tenant_isolation_policy
    ON agent_departments
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant')::VARCHAR);

CREATE POLICY agent_interaction_logs_tenant_isolation_policy
    ON agent_interaction_logs
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant')::VARCHAR);

CREATE POLICY agent_consolidated_memory_tenant_isolation_policy
    ON agent_consolidated_memory
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant')::VARCHAR);

-- Indexes for performance
CREATE INDEX idx_agent_departments_tenant ON agent_departments(tenant_id);
CREATE INDEX idx_agent_interaction_logs_tenant_status ON agent_interaction_logs(tenant_id, status);
CREATE INDEX idx_agent_consolidated_memory_tenant ON agent_consolidated_memory(tenant_id);
CREATE INDEX idx_agent_consolidated_memory_embedding ON agent_consolidated_memory USING ivfflat (embedding vector_cosine_ops);
