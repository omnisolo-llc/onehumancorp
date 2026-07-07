-- +goose Up
CREATE TABLE IF NOT EXISTS ohc_staff_tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    staff_id TEXT REFERENCES ohc_staff_member(id) ON DELETE SET NULL,
    shift_id TEXT REFERENCES shifts(id) ON DELETE SET NULL,
    title TEXT NOT NULL,
    description TEXT,
    priority TEXT NOT NULL DEFAULT 'medium' CHECK (priority IN ('low', 'medium', 'high', 'urgent')),
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'in_progress', 'completed', 'escalated')),
    escalated_to TEXT,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    _sync_status TEXT NOT NULL DEFAULT 'pending',
    version INTEGER NOT NULL DEFAULT 1
);

CREATE INDEX IF NOT EXISTS idx_ohc_staff_tasks_tenant_id ON ohc_staff_tasks(tenant_id);
CREATE INDEX IF NOT EXISTS idx_ohc_staff_tasks_staff_id ON ohc_staff_tasks(staff_id);
CREATE INDEX IF NOT EXISTS idx_ohc_staff_tasks_shift_id ON ohc_staff_tasks(shift_id);

ALTER TABLE ohc_staff_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_staff_tasks ON ohc_staff_tasks;
CREATE POLICY tenant_isolation_ohc_staff_tasks ON ohc_staff_tasks
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS ohc_shift_summaries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    shift_id TEXT NOT NULL REFERENCES shifts(id) ON DELETE CASCADE,
    summary_text TEXT NOT NULL,
    issues_escalated INTEGER NOT NULL DEFAULT 0,
    tasks_completed INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ohc_shift_summaries_tenant_id ON ohc_shift_summaries(tenant_id);
CREATE INDEX IF NOT EXISTS idx_ohc_shift_summaries_shift_id ON ohc_shift_summaries(shift_id);

ALTER TABLE ohc_shift_summaries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_shift_summaries ON ohc_shift_summaries;
CREATE POLICY tenant_isolation_ohc_shift_summaries ON ohc_shift_summaries
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_ohc_shift_summaries ON ohc_shift_summaries;
DROP TABLE IF EXISTS ohc_shift_summaries CASCADE;

DROP POLICY IF EXISTS tenant_isolation_ohc_staff_tasks ON ohc_staff_tasks;
DROP TABLE IF EXISTS ohc_staff_tasks CASCADE;
