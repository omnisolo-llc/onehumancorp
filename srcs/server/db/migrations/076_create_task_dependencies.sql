-- +goose Up
CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id UUID NOT NULL,
    depends_on_task_id UUID NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id)
);

ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS mission_id TEXT;

-- +goose Down
DROP TABLE IF EXISTS task_dependencies;
ALTER TABLE shared_tasks DROP COLUMN IF NOT EXISTS mission_id;
