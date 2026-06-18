CREATE TABLE IF NOT EXISTS interaction_timeline (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    source TEXT NOT NULL,
    sentiment TEXT NOT NULL,
    occurred_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_interaction_timeline_tenant_customer ON interaction_timeline(tenant_id, customer_id);

ALTER TABLE interaction_timeline ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_interaction_timeline ON interaction_timeline;
CREATE POLICY tenant_isolation_interaction_timeline
ON interaction_timeline
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
