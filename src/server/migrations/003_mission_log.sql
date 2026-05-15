-- Migration: 003_mission_log.sql
-- Add mission_log column to agent_missions for tracking handover blockers

ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS mission_log TEXT;
