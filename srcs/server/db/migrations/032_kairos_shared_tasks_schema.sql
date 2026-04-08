-- +goose Up
ALTER TABLE shared_tasks ADD COLUMN organization_id VARCHAR NOT NULL DEFAULT '';
CREATE INDEX idx_shared_tasks_org_status ON shared_tasks(organization_id, status);

-- +goose Down
DROP INDEX idx_shared_tasks_org_status;
ALTER TABLE shared_tasks DROP COLUMN organization_id;
