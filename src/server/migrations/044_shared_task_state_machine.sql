-- 044_shared_task_state_machine.sql

-- The tasks table already exists with a status CHECK constraint.
-- It already has id, parent_task_id, agent_id, status, payload, created_at, updated_at.
-- We need to ensure it supports the new fields if any, like title and description.

ALTER TABLE tasks ADD COLUMN IF NOT EXISTS organization_id VARCHAR NOT NULL DEFAULT '';
ALTER TABLE tasks ADD COLUMN IF NOT EXISTS title VARCHAR DEFAULT '';
ALTER TABLE tasks ADD COLUMN IF NOT EXISTS description TEXT;
