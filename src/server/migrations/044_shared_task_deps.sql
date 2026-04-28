-- 044_shared_task_deps.sql

CREATE TABLE IF NOT EXISTS shared_task_dependencies (
    task_id TEXT NOT NULL,
    depends_on_task_id TEXT NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id),
    FOREIGN KEY (task_id) REFERENCES shared_tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (depends_on_task_id) REFERENCES shared_tasks(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_shared_task_deps_task ON shared_task_dependencies(task_id);
CREATE INDEX IF NOT EXISTS idx_shared_task_deps_depends ON shared_task_dependencies(depends_on_task_id);
