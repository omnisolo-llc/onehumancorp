CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    mission_id TEXT NOT NULL,
    parent_plan_id TEXT NOT NULL,
    dependencies JSONB NOT NULL DEFAULT '[]'::jsonb,
    title TEXT NOT NULL,
    description TEXT,
    assigned_agent_id TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    priority TEXT NOT NULL,
    payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    locked_until TIMESTAMPTZ,
    ultraplan_phase TEXT,
    deliberation_log JSONB NOT NULL DEFAULT '[]'::jsonb,
    depth INT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    action_risk TEXT,
    approval_status TEXT,
    proposed_content TEXT
);

CREATE TABLE IF NOT EXISTS state_machine_transitions (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL REFERENCES shared_tasks_decomposition(id),
    from_state TEXT NOT NULL,
    to_state TEXT NOT NULL,
    agent_id TEXT,
    transitioned_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
ALTER TABLE shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition USING (tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
ALTER TABLE state_machine_transitions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_state_machine_transitions ON state_machine_transitions USING (task_id IN (SELECT id FROM shared_tasks_decomposition WHERE tenant_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system'));
