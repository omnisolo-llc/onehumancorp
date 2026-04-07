ALTER TABLE shared_tasks ADD COLUMN organization_id VARCHAR NOT NULL DEFAULT 'default-org';
CREATE INDEX IF NOT EXISTS idx_shared_tasks_org_status ON shared_tasks(organization_id, status);
