-- +goose Up
-- 051_kairos_phase1.sql
-- Ensure KAIROS Phase 1 shared_tasks specific columns are present.

ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS agent_id VARCHAR(255);
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS assigned_agent VARCHAR;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS assigned_agent_id TEXT;

-- +goose Down
-- Revert added columns
ALTER TABLE shared_tasks DROP COLUMN IF EXISTS agent_id;
ALTER TABLE shared_tasks DROP COLUMN IF EXISTS assigned_agent;
ALTER TABLE shared_tasks DROP COLUMN IF EXISTS assigned_agent_id;
