-- 008_swarm_tasks.sql
-- KAIROS Orchestrator: Shared Task List & Teammate Mesh Coordination System

CREATE TABLE IF NOT EXISTS swarm_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    mission_id TEXT REFERENCES agent_missions(id),
    parent_task_id TEXT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL CHECK (status IN ('PENDING', 'IN_PROGRESS', 'BLOCKED', 'COMPLETED', 'FAILED')),
    assigned_agent_id TEXT,
    dependencies JSONB,
    locked_until TIMESTAMPTZ,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- For autoDream Memory Embeddings
CREATE TABLE IF NOT EXISTS swarm_long_term_memory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    topic TEXT NOT NULL,
    summary TEXT NOT NULL,
    embedding VECTOR(1536), -- pgvector
    created_at TIMESTAMPTZ DEFAULT NOW()
);
