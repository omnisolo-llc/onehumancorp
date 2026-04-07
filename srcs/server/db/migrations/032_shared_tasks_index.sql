-- In sqlite we cannot add a column with NOT NULL and no constant default.
-- Disable IF NOT EXISTS to ensure compatibility
ALTER TABLE shared_tasks ADD COLUMN organization_id VARCHAR DEFAULT 'system';
CREATE INDEX IF NOT EXISTS idx_shared_tasks_org_status ON shared_tasks(organization_id, status);
