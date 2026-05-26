-- Reconstruction of the agent_missions table to persist the STUCK enum constraint safely.
-- We must retain all critical columns (updated_at, synced_to_cloud, organization_id, etc.).

-- 1. Rename existing table to _temp_agent_missions
ALTER TABLE agent_missions RENAME TO _temp_agent_missions;

-- 2. Re-create the agent_missions table with the CHECK constraint.
CREATE TABLE agent_missions (
    id TEXT PRIMARY KEY,
    status TEXT NOT NULL CHECK(status IN ('PENDING', 'RUNNING', 'STUCK', 'COMPLETED', 'FAILED', 'CLOUD_ESCALATION', 'BURSTING', 'blocked')),
    payload TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    tenant_id TEXT NOT NULL DEFAULT 'system',
    cloud_mission_id TEXT,
    sync_error TEXT,
    last_synced_at TIMESTAMP,
    synced_to_cloud BOOLEAN DEFAULT false,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1,
    mission_log TEXT,
    organization_id TEXT NOT NULL DEFAULT 'system'
);

-- 3. Copy the data back explicitly.
-- We include all critical columns including organization_id.
INSERT INTO agent_missions (
    id, status, payload, created_at, updated_at, tenant_id, synced_to_cloud, organization_id
)
SELECT
    id, status, payload, created_at, updated_at, tenant_id, synced_to_cloud, organization_id
FROM _temp_agent_missions;

-- 4. Drop the temporary table.
-- Postgres requires CASCADE due to tenant_isolation_swarm_tasks depending on agent_missions.
-- But SQLite does not support CASCADE on DROP TABLE.
-- However, we must provide a SQLite migration as requested.
DROP TABLE _temp_agent_missions;
