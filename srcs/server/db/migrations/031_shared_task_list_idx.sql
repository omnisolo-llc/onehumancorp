-- We must add `organization_id` to `shared_tasks` since an early migration created it without the column.
ALTER TABLE shared_tasks ADD COLUMN organization_id VARCHAR NOT NULL DEFAULT 'default-org';
CREATE INDEX IF NOT EXISTS idx_shared_tasks_org_status ON shared_tasks(organization_id, status);
