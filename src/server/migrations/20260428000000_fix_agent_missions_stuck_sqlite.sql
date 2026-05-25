-- Create new table with correct constraint
CREATE TABLE agent_missions_new (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    status TEXT NOT NULL CHECK (status IN ('PENDING', 'IN_PROGRESS', 'COMPLETED', 'FAILED', 'STUCK')),
    payload TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    synced_to_cloud BOOLEAN DEFAULT false,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    organization_id TEXT
);

-- Copy data
INSERT INTO agent_missions_new (id, tenant_id, status, payload, created_at, synced_to_cloud, updated_at, organization_id)
SELECT id, tenant_id, status, payload, created_at, synced_to_cloud, updated_at, organization_id
FROM agent_missions;

-- Drop old table
DROP TABLE agent_missions;

-- Rename new table to original
ALTER TABLE agent_missions_new RENAME TO agent_missions;

-- Recreate policies and indexes if needed
ALTER TABLE agent_missions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_missions ON agent_missions USING (tenant_id::text = current_setting('app.current_tenant', true));
