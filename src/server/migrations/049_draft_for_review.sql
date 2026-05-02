ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS action_risk VARCHAR;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS approval_status VARCHAR;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS proposed_content TEXT;
