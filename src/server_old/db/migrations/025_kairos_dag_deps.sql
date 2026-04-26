-- Disable IF NOT EXISTS since sqlite might not support it for all alter statements in this version
ALTER TABLE shared_tasks ADD COLUMN parent_plan_id TEXT;
ALTER TABLE shared_tasks ADD COLUMN dependencies JSONB NOT NULL DEFAULT '[]';

CREATE TABLE swarm_task_dependencies (
    task_id UUID REFERENCES swarm_tasks(id) ON DELETE CASCADE,
    depends_on_task_id UUID REFERENCES swarm_tasks(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, depends_on_task_id)
);
CREATE INDEX idx_swarm_task_deps_task ON swarm_task_dependencies(task_id);
CREATE INDEX idx_swarm_task_deps_depends ON swarm_task_dependencies(depends_on_task_id);
