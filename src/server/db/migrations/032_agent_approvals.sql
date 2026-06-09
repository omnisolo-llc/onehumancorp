-- Migration: 032_agent_approvals.sql
-- Real-Time Operational Agent Feed & Triage

CREATE TYPE agent_department_enum AS ENUM (
    'Operations',
    'CustomerSuccess',
    'Sales',
    'Marketing',
    'Finance'
);

CREATE TYPE action_priority_enum AS ENUM (
    'High',
    'Medium',
    'Low'
);

CREATE TYPE action_status_enum AS ENUM (
    'Pending',
    'Approved',
    'Rejected',
    'Dismissed'
);

CREATE TABLE agent_approvals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    agent_department agent_department_enum NOT NULL,
    priority action_priority_enum NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    status action_status_enum NOT NULL DEFAULT 'Pending',
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    decided_at TIMESTAMPTZ
);

ALTER TABLE agent_approvals ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_policy ON agent_approvals
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant')::UUID);

CREATE INDEX idx_agent_approvals_tenant_status ON agent_approvals(tenant_id, status);
CREATE INDEX idx_agent_approvals_created_at ON agent_approvals(created_at DESC);
