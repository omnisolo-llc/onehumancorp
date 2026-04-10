ALTER TABLE shared_tasks ADD COLUMN organization_id VARCHAR DEFAULT 'default_org';
CREATE INDEX IF NOT EXISTS idx_shared_tasks_org_status ON shared_tasks(organization_id, status);
