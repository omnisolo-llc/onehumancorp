-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS action_risk TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS approval_status TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS proposed_content TEXT;

ALTER TABLE shared_tasks_master ADD COLUMN IF NOT EXISTS action_risk TEXT;
ALTER TABLE shared_tasks_master ADD COLUMN IF NOT EXISTS approval_status TEXT;
ALTER TABLE shared_tasks_master ADD COLUMN IF NOT EXISTS proposed_content TEXT;

ALTER TABLE shared_tasks_v2 ADD COLUMN IF NOT EXISTS action_risk TEXT;
ALTER TABLE shared_tasks_v2 ADD COLUMN IF NOT EXISTS approval_status TEXT;
ALTER TABLE shared_tasks_v2 ADD COLUMN IF NOT EXISTS proposed_content TEXT;

ALTER TABLE shared_tasks_v4 ADD COLUMN IF NOT EXISTS action_risk TEXT;
ALTER TABLE shared_tasks_v4 ADD COLUMN IF NOT EXISTS approval_status TEXT;
ALTER TABLE shared_tasks_v4 ADD COLUMN IF NOT EXISTS proposed_content TEXT;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE shared_tasks DROP COLUMN IF EXISTS action_risk;
ALTER TABLE shared_tasks DROP COLUMN IF EXISTS approval_status;
ALTER TABLE shared_tasks DROP COLUMN IF EXISTS proposed_content;

ALTER TABLE shared_tasks_master DROP COLUMN IF EXISTS action_risk;
ALTER TABLE shared_tasks_master DROP COLUMN IF EXISTS approval_status;
ALTER TABLE shared_tasks_master DROP COLUMN IF EXISTS proposed_content;

ALTER TABLE shared_tasks_v2 DROP COLUMN IF EXISTS action_risk;
ALTER TABLE shared_tasks_v2 DROP COLUMN IF EXISTS approval_status;
ALTER TABLE shared_tasks_v2 DROP COLUMN IF EXISTS proposed_content;

ALTER TABLE shared_tasks_v4 DROP COLUMN IF EXISTS action_risk;
ALTER TABLE shared_tasks_v4 DROP COLUMN IF EXISTS approval_status;
ALTER TABLE shared_tasks_v4 DROP COLUMN IF EXISTS proposed_content;
-- +goose StatementEnd
