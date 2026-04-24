-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN dependencies TEXT NOT NULL DEFAULT '[]';

DROP TABLE IF EXISTS task_dependencies;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id TEXT NOT NULL,
    depends_on_task_id TEXT NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id)
);

INSERT INTO task_dependencies (task_id, depends_on_task_id)
SELECT id, value
FROM shared_tasks, json_each(dependencies);

ALTER TABLE shared_tasks DROP COLUMN dependencies;
-- +goose StatementEnd
