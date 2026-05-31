-- KAIROS Shared Task List
CREATE TABLE IF NOT EXISTS shared_tasks_v5 (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    agent_id TEXT,
    priority INTEGER,
    payload JSONB,
    parent_plan_id TEXT,
    dependencies JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    ultraplan_phase TEXT,
    deliberation_log JSONB DEFAULT '[]',
    depth INTEGER
);

CREATE TABLE IF NOT EXISTS state_machine_transitions_v5 (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL,
    from_state TEXT NOT NULL,
    to_state TEXT NOT NULL,
    agent_id TEXT,
    transitioned_at TIMESTAMP WITH TIME ZONE NOT NULL
);
