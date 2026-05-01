-- 20260428000000_fix_agent_missions_stuck_sqlite.sql
-- Reconstruct the agent_missions table to safely persist the STUCK enum and retain critical columns

-- 1. Create a new table with the correct schema
CREATE TABLE IF NOT EXISTS agent_missions_new (
    id         TEXT PRIMARY KEY,
    status     TEXT NOT NULL,
    payload    TEXT NOT NULL,
    organization_id TEXT DEFAULT 'system',
    synced_to_cloud BOOLEAN DEFAULT false,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    CHECK(status IN ('PENDING', 'RUNNING', 'COMPLETED', 'FAILED', 'STUCK', 'pending', 'running', 'completed', 'failed', 'stuck'))
);

-- 2. Copy data from the old table to the new table
INSERT INTO agent_missions_new (id, status, payload, organization_id, synced_to_cloud, created_at, updated_at)
SELECT id, status, payload, organization_id, synced_to_cloud, created_at, updated_at
FROM agent_missions;

-- 3. Drop the old table
DROP TABLE agent_missions;

-- 4. Rename the new table to the old table's name
ALTER TABLE agent_missions_new RENAME TO agent_missions;

-- 5. Recreate indexes
CREATE INDEX idx_missions_status ON agent_missions (status);
CREATE INDEX idx_agent_missions_org_status ON agent_missions(organization_id, status);
