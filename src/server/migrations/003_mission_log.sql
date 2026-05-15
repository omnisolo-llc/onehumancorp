-- Migration: 003_mission_log.sql
-- Add mission_log column to agent_missions to store blockers and status updates

ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS mission_log TEXT;
