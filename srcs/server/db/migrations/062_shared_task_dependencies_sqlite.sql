-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id TEXT NOT NULL,
    depends_on_task_id TEXT NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id)
);

INSERT OR IGNORE INTO task_dependencies (task_id, depends_on_task_id)
SELECT shared_tasks.id, value
FROM shared_tasks, json_each(dependencies)
/* ON CONFLICT DO NOTHING */;

ALTER TABLE shared_tasks DROP COLUMN dependencies;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN dependencies TEXT NOT NULL DEFAULT '[]';

WITH RECURSIVE deps AS (
    SELECT task_id, json_group_array(depends_on_task_id) as dep_arr
    FROM task_dependencies
    GROUP BY task_id
)
UPDATE shared_tasks
SET dependencies = (SELECT dep_arr FROM deps WHERE deps.task_id = shared_tasks.id)
WHERE EXISTS (SELECT 1 FROM deps WHERE deps.task_id = shared_tasks.id);

DROP TABLE IF EXISTS task_dependencies;
-- +goose StatementEnd
