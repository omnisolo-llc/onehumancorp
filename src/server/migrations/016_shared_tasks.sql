-- 016_shared_tasks.sql
-- Upgrade shared_tasks table from Go migration 013

ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS tenant_id VARCHAR NOT NULL DEFAULT '';
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS agent_id VARCHAR;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS payload JSONB;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS locked_until TIMESTAMP WITH TIME ZONE;
