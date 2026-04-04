CREATE TABLE IF NOT EXISTS swarm_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    mission_id TEXT NOT NULL,
    parent_plan_id TEXT,
    dependencies JSONB NOT NULL DEFAULT '[]',
    title TEXT NOT NULL,
    description TEXT,
    priority TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'IN_PROGRESS', 'COMPLETED', 'FAILED')),
    assigned_agent_id TEXT,
    locked_until TIMESTAMPTZ,
    payload JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE EXTENSION IF NOT EXISTS vector;

-- For autoDream Memory Embeddings
CREATE TABLE IF NOT EXISTS swarm_long_term_memory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    topic TEXT NOT NULL,
    summary TEXT NOT NULL,
    embedding VECTOR(1536),
    created_at TIMESTAMPTZ DEFAULT NOW()
);
