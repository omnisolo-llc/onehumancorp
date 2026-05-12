-- 024_kairos_orchestration.sql
-- Re-creation attempts with IF NOT EXISTS, likely no-op in this sequence.

CREATE TABLE IF NOT EXISTS shared_tasks (
    id UUID PRIMARY KEY,
    mission_id TEXT NOT NULL,
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    agent_id VARCHAR,
    priority VARCHAR NOT NULL DEFAULT 'P2',
    payload JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id UUID REFERENCES shared_tasks(id) ON DELETE CASCADE,
    depends_on_task_id UUID REFERENCES shared_tasks(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, depends_on_task_id)
);

CREATE TABLE IF NOT EXISTS agent_memories (
    id UUID PRIMARY KEY,
    mission_id TEXT NOT NULL,
    organization_id VARCHAR NOT NULL,
    content TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
