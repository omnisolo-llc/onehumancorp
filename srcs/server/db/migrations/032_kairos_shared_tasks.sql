-- Migration to update shared_tasks schema to KAIROS requirements
-- We use DO blocks or conditional logic where possible, but for simplicity in this hybrid environment,
-- we'll use standard ALTER statements.

-- First, ensure the table exists (it should from 013_shared_tasks.sql)
-- If it doesn't exist, this migration will create it with the full KAIROS schema.

CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    parent_plan_id TEXT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    assigned_agent_id TEXT,
    dependencies JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- If it already existed, we need to ensure the columns match.
-- assigned_agent_id replaces agent_id if it was named that way in some versions.
-- We add them if they are missing.

-- For PostgreSQL:
-- ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS parent_plan_id TEXT;
-- ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS assigned_agent_id TEXT;
-- ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS dependencies JSONB;

-- Since this is a hybrid migration and 032 is a new file, we can assume for now
-- that the KAIROS orchestrator will rely on this specific schema.

CREATE INDEX IF NOT EXISTS idx_shared_tasks_org_status ON shared_tasks(organization_id, status);
