CREATE TABLE IF NOT EXISTS work_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    type VARCHAR(50) NOT NULL, -- message, task, alert
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending, drafted, completed, archived
    title TEXT NOT NULL,
    preview TEXT,
    draft_response TEXT,
    payload JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_work_items_tenant_status ON work_items(tenant_id, status);

ALTER TABLE work_items ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_work_items ON work_items
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
