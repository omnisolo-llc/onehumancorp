-- Postgres migration for KAIROS Master Blueprint

CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS kairos_shared_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    parent_task_id UUID REFERENCES kairos_shared_tasks(id),
    epic_id VARCHAR,
    tenant_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    assigned_agent_id VARCHAR,
    priority VARCHAR NOT NULL DEFAULT 'P2',
    payload JSONB,
    dependencies JSONB NOT NULL DEFAULT '[]',
    locked_until TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE kairos_shared_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_policy ON kairos_shared_tasks
    USING (tenant_id = current_setting('app.current_tenant_id', true));

CREATE TABLE IF NOT EXISTS kairos_state_transitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID REFERENCES kairos_shared_tasks(id),
    tenant_id VARCHAR NOT NULL,
    from_state VARCHAR NOT NULL,
    to_state VARCHAR NOT NULL,
    agent_id VARCHAR NOT NULL,
    reason TEXT,
    occurred_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE kairos_state_transitions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_policy ON kairos_state_transitions
    USING (tenant_id = current_setting('app.current_tenant_id', true));

CREATE TABLE IF NOT EXISTS kairos_sub_agent_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR NOT NULL,
    parent_task_id UUID REFERENCES kairos_shared_tasks(id),
    payload JSONB,
    status VARCHAR NOT NULL DEFAULT 'QUEUED',
    worker_id VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE kairos_sub_agent_jobs ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_policy ON kairos_sub_agent_jobs
    USING (tenant_id = current_setting('app.current_tenant_id', true));

CREATE TABLE IF NOT EXISTS autodream_vector_memories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID REFERENCES kairos_shared_tasks(id),
    tenant_id VARCHAR NOT NULL,
    content TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE autodream_vector_memories ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_policy ON autodream_vector_memories
    USING (tenant_id = current_setting('app.current_tenant_id', true));
