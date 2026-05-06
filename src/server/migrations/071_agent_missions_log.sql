-- 071_agent_missions_log.sql
-- Add mission_log column to agent_missions to store handover blocker logs.

ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS mission_log TEXT;
