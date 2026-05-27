ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS approval_status VARCHAR(255);
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS proposed_content TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS action_risk VARCHAR(50);
