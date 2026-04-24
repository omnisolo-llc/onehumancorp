-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS dependencies JSONB NOT NULL DEFAULT '[]';

DROP TABLE IF EXISTS task_dependencies;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id UUID NOT NULL,
    depends_on_task_id UUID NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id)
);

INSERT INTO task_dependencies (task_id, depends_on_task_id)
SELECT id, value::text::uuid
FROM shared_tasks, jsonb_array_elements_text(dependencies) AS value;

ALTER TABLE shared_tasks DROP COLUMN dependencies;
-- +goose StatementEnd
