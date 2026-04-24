-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS dependencies JSONB NOT NULL DEFAULT '[]';

DO $$
BEGIN
    IF EXISTS (
        SELECT FROM information_schema.tables
        WHERE table_schema = 'public'
        AND table_name   = 'task_dependencies'
    ) THEN
        WITH deps AS (
            SELECT task_id, jsonb_agg(depends_on_task_id) as dep_arr
            FROM task_dependencies
            GROUP BY task_id
        )
        UPDATE shared_tasks
        SET dependencies = deps.dep_arr
        FROM deps
        WHERE deps.task_id::text = shared_tasks.id;
    END IF;
END $$;

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
