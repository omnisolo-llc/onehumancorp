-- +goose Up
-- +goose StatementBegin
DROP TABLE IF EXISTS shared_tasks;
ALTER TABLE swarm_tasks RENAME TO shared_tasks;

DROP TABLE IF EXISTS task_dependencies;
ALTER TABLE swarm_task_dependencies RENAME TO task_dependencies;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE shared_tasks RENAME TO swarm_tasks;
ALTER TABLE task_dependencies RENAME TO swarm_task_dependencies;
-- +goose StatementEnd
