CREATE TABLE IF NOT EXISTS waitlist_entries (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL REFERENCES organizations(id),
    customer_id TEXT NOT NULL,
    resource_id TEXT NOT NULL,
    intent_timestamp BIGINT NOT NULL,
    priority_score INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_waitlist_entries_org ON waitlist_entries(organization_id);
CREATE INDEX IF NOT EXISTS idx_waitlist_entries_resource ON waitlist_entries(organization_id, resource_id, status, priority_score DESC, intent_timestamp ASC);

ALTER TABLE waitlist_entries ENABLE ROW LEVEL SECURITY;

CREATE POLICY waitlist_entries_tenant_isolation_policy ON waitlist_entries
    USING (organization_id = current_setting('app.current_tenant', true));
