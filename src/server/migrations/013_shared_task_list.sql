ALTER TABLE tasks ADD COLUMN epic_id TEXT;
ALTER TABLE tasks ADD COLUMN locked_by TEXT REFERENCES agents(id);
ALTER TABLE tasks ADD COLUMN locked_at TIMESTAMPTZ;

CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id TEXT REFERENCES tasks(id) ON DELETE CASCADE,
    depends_on_task_id TEXT REFERENCES tasks(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, depends_on_task_id)
);
