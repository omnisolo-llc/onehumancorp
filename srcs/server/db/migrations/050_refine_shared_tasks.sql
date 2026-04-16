-- +goose Up
-- +goose StatementBegin
ALTER TABLE swarm_tasks ADD COLUMN ultraplan_phase VARCHAR;
ALTER TABLE swarm_tasks ADD COLUMN deliberation_log TEXT;
ALTER TABLE swarm_tasks ADD COLUMN depth INTEGER DEFAULT 0;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE swarm_tasks DROP COLUMN depth;
ALTER TABLE swarm_tasks DROP COLUMN deliberation_log;
ALTER TABLE swarm_tasks DROP COLUMN ultraplan_phase;
-- +goose StatementEnd
