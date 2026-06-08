CREATE TABLE IF NOT EXISTS triage_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    source TEXT,
    priority TEXT,
    context TEXT,
    status TEXT DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS triage_proposed_actions (
    id TEXT PRIMARY KEY,
    triage_item_id TEXT NOT NULL REFERENCES triage_items(id) ON DELETE CASCADE,
    tenant_id TEXT NOT NULL,
    action_type TEXT,
    payload TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_triage_items_tenant_status ON triage_items(tenant_id, status);

ALTER TABLE triage_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_triage_items ON triage_items;
CREATE POLICY tenant_isolation_triage_items ON triage_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE triage_proposed_actions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_triage_actions ON triage_proposed_actions;
CREATE POLICY tenant_isolation_triage_actions ON triage_proposed_actions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
