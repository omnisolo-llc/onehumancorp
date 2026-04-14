-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS ultraplan_phase VARCHAR;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS deliberation_log TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS depth INTEGER DEFAULT 0;

ALTER TABLE shared_tasks_master ADD COLUMN IF NOT EXISTS ultraplan_phase VARCHAR;
ALTER TABLE shared_tasks_master ADD COLUMN IF NOT EXISTS deliberation_log TEXT;
ALTER TABLE shared_tasks_master ADD COLUMN IF NOT EXISTS depth INTEGER DEFAULT 0;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE shared_tasks DROP COLUMN IF EXISTS depth;
ALTER TABLE shared_tasks DROP COLUMN IF EXISTS deliberation_log;
ALTER TABLE shared_tasks DROP COLUMN IF EXISTS ultraplan_phase;

ALTER TABLE shared_tasks_master DROP COLUMN IF EXISTS depth;
ALTER TABLE shared_tasks_master DROP COLUMN IF EXISTS deliberation_log;
ALTER TABLE shared_tasks_master DROP COLUMN IF EXISTS ultraplan_phase;
-- +goose StatementEnd
