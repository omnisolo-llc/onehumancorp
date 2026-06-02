-- Create vector extension if not exists
CREATE EXTENSION IF NOT EXISTS vector;

-- Create agent_memory table
CREATE TABLE agent_memory (
    id SERIAL PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    department VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    embedding vector(1536), -- Assuming standard 1536 dims for openai embeddings
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS
ALTER TABLE agent_memory ENABLE ROW LEVEL SECURITY;

-- Create RLS policy ensuring access is restricted to the current tenant
CREATE POLICY tenant_isolation_policy ON agent_memory
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id'));

-- Add indexes for faster similarity search and tenant lookup
CREATE INDEX ON agent_memory USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);
CREATE INDEX idx_agent_memory_tenant_dept ON agent_memory(tenant_id, department);
