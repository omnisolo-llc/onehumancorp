-- Migration: AI-Powered Staff Task & Shift Coordination
-- Data Model for Staff Tasks, Shifts, and Assignments

CREATE TABLE IF NOT EXISTS staff_shifts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE staff_shifts ENABLE ROW LEVEL SECURITY;
CREATE POLICY staff_shifts_tenant_isolation ON staff_shifts USING (tenant_id = current_setting('app.current_tenant')::UUID);

CREATE TABLE IF NOT EXISTS staff_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    role TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE staff_members ENABLE ROW LEVEL SECURITY;
CREATE POLICY staff_members_tenant_isolation ON staff_members USING (tenant_id = current_setting('app.current_tenant')::UUID);

CREATE TABLE IF NOT EXISTS staff_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    priority TEXT NOT NULL DEFAULT 'Medium',
    deadline TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE staff_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY staff_tasks_tenant_isolation ON staff_tasks USING (tenant_id = current_setting('app.current_tenant')::UUID);

CREATE TABLE IF NOT EXISTS staff_task_assignments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    task_id UUID NOT NULL REFERENCES staff_tasks(id) ON DELETE CASCADE,
    staff_id UUID REFERENCES staff_members(id) ON DELETE SET NULL,
    shift_id UUID REFERENCES staff_shifts(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE staff_task_assignments ENABLE ROW LEVEL SECURITY;
CREATE POLICY staff_task_assignments_tenant_isolation ON staff_task_assignments USING (tenant_id = current_setting('app.current_tenant')::UUID);
