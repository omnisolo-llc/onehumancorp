-- 028_kairos_dag_deps.sql
-- Upgrade shared_tasks and create swarm_task_dependencies

ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS parent_plan_id TEXT;

-- Alter column type to JSONB (drop default first, cast, then set new default)
ALTER TABLE shared_tasks ALTER COLUMN dependencies DROP DEFAULT;
ALTER TABLE shared_tasks ALTER COLUMN dependencies TYPE JSONB USING dependencies::JSONB;
ALTER TABLE shared_tasks ALTER COLUMN dependencies SET DEFAULT '[]'::JSONB;

CREATE TABLE IF NOT EXISTS swarm_task_dependencies (
    task_id UUID REFERENCES swarm_tasks(id) ON DELETE CASCADE,
    depends_on_task_id UUID REFERENCES swarm_tasks(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, depends_on_task_id)
);

CREATE INDEX IF NOT EXISTS idx_swarm_task_deps_task ON swarm_task_dependencies(task_id);
CREATE INDEX IF NOT EXISTS idx_swarm_task_deps_depends ON swarm_task_dependencies(depends_on_task_id);
