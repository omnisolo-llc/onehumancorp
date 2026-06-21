-- Migration 147: Add sync_error to sub_agent_queue for consistent error tracking

ALTER TABLE sub_agent_queue ADD COLUMN IF NOT EXISTS sync_error TEXT;
