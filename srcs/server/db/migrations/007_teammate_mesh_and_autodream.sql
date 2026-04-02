-- 007_teammate_mesh_and_autodream.sql
-- Create Shared Task List and autoDream Vector Memory tables

CREATE TABLE IF NOT EXISTS shared_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    mission_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    assigned_agent_id TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    priority TEXT NOT NULL DEFAULT 'P2',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_shared_tasks_status ON shared_tasks(status);

CREATE TABLE IF NOT EXISTS autodream_memories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content TEXT NOT NULL,
    embedding vector(1536),
    source_mission_id TEXT UNIQUE,
    consolidated_at TIMESTAMPTZ DEFAULT NOW()
);
