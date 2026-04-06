-- SQLite does not support ADD COLUMN IF NOT EXISTS or CREATE EXTENSION
-- Postgres does, but for SQLite compatibility in testing we use simple ADD COLUMN.
-- However, SQLite will error if the column already exists.
-- Since this is an unreleased migration in our local environment, we can just drop/recreate,
-- or use proper SQLite compatible syntax.

-- Because we just added autodream_memories in 024, let's just alter it simply.
-- Actually, SQLite ALTER TABLE ADD COLUMN does not support IF NOT EXISTS.

ALTER TABLE autodream_memories ADD COLUMN organization_id TEXT;
ALTER TABLE autodream_memories ADD COLUMN agent_id TEXT;
ALTER TABLE autodream_memories ADD COLUMN source_type TEXT;

UPDATE autodream_memories SET organization_id = 'default' WHERE organization_id IS NULL;
UPDATE autodream_memories SET source_type = 'unknown' WHERE source_type IS NULL;

CREATE INDEX IF NOT EXISTS idx_autodream_org ON autodream_memories(organization_id);
