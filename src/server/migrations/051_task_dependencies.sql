CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id TEXT REFERENCES tasks(id) ON DELETE CASCADE,
    depends_on_task_id TEXT REFERENCES tasks(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, depends_on_task_id)
);
