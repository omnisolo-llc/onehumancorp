-- 023_shared_tasks_dependencies.sql

-- Ensure task_dependencies exists as requested in the mission
CREATE TABLE IF NOT EXISTS shared_tasks (
    id UUID PRIMARY KEY,
    mission_id TEXT NOT NULL,
    parent_plan_id TEXT,
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    assigned_agent_id TEXT,
    payload JSONB
);

CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id UUID NOT NULL,
    depends_on_task_id UUID NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id)
);
