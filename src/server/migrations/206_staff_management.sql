-- +goose Up
CREATE TABLE IF NOT EXISTS staff_tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    staff_id TEXT REFERENCES ohc_staff_member(id) ON DELETE SET NULL,
    shift_id TEXT REFERENCES shifts(id) ON DELETE SET NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    priority TEXT NOT NULL DEFAULT 'normal',
    is_offline_capable BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);

CREATE INDEX IF NOT EXISTS idx_staff_tasks_tenant_id ON staff_tasks(tenant_id);
CREATE INDEX IF NOT EXISTS idx_staff_tasks_staff_id ON staff_tasks(staff_id);

ALTER TABLE staff_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_staff_tasks ON staff_tasks;
CREATE POLICY tenant_isolation_staff_tasks
ON staff_tasks
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS shift_summaries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    shift_id TEXT REFERENCES shifts(id) ON DELETE CASCADE,
    summary_text TEXT NOT NULL,
    escalated_issues JSONB DEFAULT '[]',
    supply_shortages JSONB DEFAULT '[]',
    generated_by_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1
);

CREATE INDEX IF NOT EXISTS idx_shift_summaries_tenant_id ON shift_summaries(tenant_id);

ALTER TABLE shift_summaries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shift_summaries ON shift_summaries;
CREATE POLICY tenant_isolation_shift_summaries
ON shift_summaries
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_shift_summaries ON shift_summaries;
DROP TABLE IF EXISTS shift_summaries CASCADE;

DROP POLICY IF EXISTS tenant_isolation_staff_tasks ON staff_tasks;
DROP TABLE IF EXISTS staff_tasks CASCADE;
