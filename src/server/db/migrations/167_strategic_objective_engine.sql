-- OHC Strategic Objective Engine Migration
-- Part of P1: Proactive Cross-Agent Goal Execution

CREATE TABLE IF NOT EXISTS strategic_objectives (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    goal TEXT NOT NULL,
    target_date TIMESTAMPTZ,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS plan_tasks (
    id UUID PRIMARY KEY,
    objective_id UUID NOT NULL REFERENCES strategic_objectives(id) ON DELETE CASCADE,
    tenant_id TEXT NOT NULL,
    department TEXT NOT NULL,
    description TEXT NOT NULL,
    payload JSONB NOT NULL DEFAULT '{}',
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS
ALTER TABLE strategic_objectives ENABLE ROW LEVEL SECURITY;
ALTER TABLE plan_tasks ENABLE ROW LEVEL SECURITY;

-- Tenant Isolation Policies
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_strategic_objectives') THEN
        CREATE POLICY tenant_isolation_strategic_objectives ON strategic_objectives
            USING (tenant_id = current_setting('app.current_tenant', true));
    END IF;

    IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'tenant_isolation_plan_tasks') THEN
        CREATE POLICY tenant_isolation_plan_tasks ON plan_tasks
            USING (tenant_id = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_strategic_objectives_tenant_id ON strategic_objectives(tenant_id);
CREATE INDEX IF NOT EXISTS idx_plan_tasks_objective_id ON plan_tasks(objective_id);
CREATE INDEX IF NOT EXISTS idx_plan_tasks_tenant_id ON plan_tasks(tenant_id);
