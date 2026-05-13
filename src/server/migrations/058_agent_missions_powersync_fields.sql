-- 058_agent_missions_powersync_fields.sql
-- Add PowerSync fields to agent_missions for Cloud-to-Standalone sync.

ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS _sync_status TEXT DEFAULT 'pending';
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS version INTEGER DEFAULT 1;
