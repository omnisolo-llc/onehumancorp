-- Migration: 003_agent_missions_columns.sql
-- Align PostgreSQL agent_missions schema with SQLite for Hybrid-mode and Sync Daemon.

ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS cloud_mission_id TEXT;
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS sync_error TEXT;
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS last_synced_at TIMESTAMPTZ;
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS _sync_status TEXT DEFAULT 'pending';
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS version INTEGER DEFAULT 1;
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS mission_log TEXT;
