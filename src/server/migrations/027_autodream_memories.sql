-- 027_autodream_memories.sql
-- Upgrade autodream_memories from Go migration 024

ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS organization_id TEXT;
ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS task_id TEXT;
ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS source_type TEXT;
