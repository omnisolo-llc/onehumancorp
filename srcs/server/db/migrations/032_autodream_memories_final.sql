CREATE EXTENSION IF NOT EXISTS vector;
ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS organization_id TEXT;
ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS agent_id TEXT;
ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS source_type TEXT;
CREATE INDEX IF NOT EXISTS idx_autodream_org ON autodream_memories(organization_id);
