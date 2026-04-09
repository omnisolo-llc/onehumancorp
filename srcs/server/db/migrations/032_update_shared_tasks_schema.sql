-- In 025_kairos_dag_deps.sql they already added parent_plan_id and dependencies.
-- But wait!
-- If 007 creates shared_tasks without organization_id, then 013 (which has organization_id) is ignored by CREATE TABLE IF NOT EXISTS.
-- So we DO need to add organization_id.
-- But we DON'T need to add parent_plan_id and dependencies, because 025 adds them!
-- But wait, what if 007 creates it, and 025 adds them, so they exist.
-- What about dependencies in 025? It adds them.
-- So the ONLY missing thing is organization_id and the index on it!

ALTER TABLE shared_tasks ADD COLUMN organization_id VARCHAR DEFAULT 'default_org';
CREATE INDEX IF NOT EXISTS idx_shared_tasks_org_status ON shared_tasks(organization_id, status);
