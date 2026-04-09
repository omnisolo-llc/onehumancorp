-- Rename agent_id to assigned_agent_id to align with KAIROS Phase 1 Design Doc
ALTER TABLE shared_tasks RENAME COLUMN agent_id TO assigned_agent_id;

-- Add index for optimized status-based lookups
CREATE INDEX IF NOT EXISTS idx_shared_tasks_org_status ON shared_tasks(organization_id, status);
