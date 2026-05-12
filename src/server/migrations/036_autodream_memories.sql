-- 036_autodream_memories.sql
-- Upgrade autodream_memories from Go migration 029

ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS agent_id TEXT;

UPDATE autodream_memories SET organization_id = 'default' WHERE organization_id IS NULL;
UPDATE autodream_memories SET source_type = 'unknown' WHERE source_type IS NULL;

CREATE INDEX IF NOT EXISTS idx_autodream_org ON autodream_memories(organization_id);
