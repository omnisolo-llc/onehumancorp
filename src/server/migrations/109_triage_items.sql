CREATE TABLE IF NOT EXISTS triage_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    source TEXT NOT NULL,
    content TEXT NOT NULL,
    priority TEXT NOT NULL,
    draft_reply TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_triage_items_tenant_id_status ON triage_items(tenant_id, status);

ALTER TABLE triage_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_triage_items ON triage_items;
CREATE POLICY tenant_isolation_triage_items ON triage_items
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS proposed_actions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    triage_item_id TEXT NOT NULL REFERENCES triage_items(id) ON DELETE CASCADE,
    action_type TEXT NOT NULL,
    payload TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_proposed_actions_tenant_triage ON proposed_actions(tenant_id, triage_item_id);

ALTER TABLE proposed_actions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_proposed_actions ON proposed_actions;
CREATE POLICY tenant_isolation_proposed_actions ON proposed_actions
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
