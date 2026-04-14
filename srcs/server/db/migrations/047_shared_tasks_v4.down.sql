-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS task_dependencies_dag;
DROP TABLE IF EXISTS shared_tasks_v4;
-- +goose StatementEnd
