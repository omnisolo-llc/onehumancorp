-- +goose Up
ALTER TABLE project_tasks ADD COLUMN IF NOT EXISTS proposal_id TEXT;
CREATE INDEX IF NOT EXISTS idx_project_tasks_proposal_id ON project_tasks(proposal_id);

-- +goose Down
ALTER TABLE project_tasks DROP COLUMN IF NOT EXISTS proposal_id;
