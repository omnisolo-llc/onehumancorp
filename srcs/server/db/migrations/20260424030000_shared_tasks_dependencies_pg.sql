-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id UUID REFERENCES shared_tasks(id) ON DELETE CASCADE,
    depends_on_task_id UUID REFERENCES shared_tasks(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, depends_on_task_id)
);

ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS dependencies JSONB NOT NULL DEFAULT '[]';

WITH deps AS (
    SELECT task_id, jsonb_agg(depends_on_task_id) as dep_arr
    FROM task_dependencies
    GROUP BY task_id
)
UPDATE shared_tasks
SET dependencies = deps.dep_arr
FROM deps
WHERE deps.task_id = shared_tasks.id::uuid;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS task_dependencies;
ALTER TABLE shared_tasks DROP COLUMN dependencies;
-- +goose StatementEnd
