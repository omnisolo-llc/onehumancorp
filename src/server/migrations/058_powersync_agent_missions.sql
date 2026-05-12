-- 058_powersync_agent_missions.sql
-- Add columns required by PowerSync for hybrid synchronization

ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS _sync_status TEXT DEFAULT 'pending';
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS version INTEGER DEFAULT 1;

UPDATE agent_missions SET _sync_status = 'pending' WHERE _sync_status IS NULL;
UPDATE agent_missions SET version = 1 WHERE version IS NULL;
