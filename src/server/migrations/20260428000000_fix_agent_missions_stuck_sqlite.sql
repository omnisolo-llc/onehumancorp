-- Reconstruction of the agent_missions table to persist the STUCK enum constraint safely.

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
-- We cannot use ADD COLUMN IF NOT EXISTS in SQLite, so we rely on SQLite's weak typing and default values
-- to just insert the core columns that we are 100% sure exist in the older versions of SQLite DBs.
-- The missing columns will get their default values from the CREATE TABLE statement.
INSERT INTO agent_missions (
    id, status, payload, created_at, updated_at, tenant_id, synced_to_cloud, organization_id
)
SELECT
    id, status, payload, created_at, updated_at, tenant_id, synced_to_cloud, 'system'
FROM _temp_agent_missions;

-- 4. Drop the temporary table.
DROP TABLE _temp_agent_missions;
