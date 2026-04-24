-- +goose Up
-- Fix the SQLite schema to add 'STUCK' to the status enum constraint and restore missing columns.

PRAGMA foreign_keys=off;

BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS agent_missions_fixed (
    id         TEXT PRIMARY KEY,
    status     TEXT NOT NULL CHECK(status IN ('PENDING', 'RUNNING', 'COMPLETED', 'FAILED', 'BURSTING', 'STUCK')),
    payload    TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    organization_id TEXT DEFAULT 'system',
    synced_to_cloud BOOLEAN DEFAULT FALSE
);

INSERT INTO agent_missions_fixed (id, status, payload, created_at, updated_at, organization_id, synced_to_cloud)
SELECT id, status, payload, created_at, COALESCE(updated_at, created_at), COALESCE(organization_id, 'system'), COALESCE(synced_to_cloud, FALSE) FROM agent_missions;

DROP TABLE agent_missions;
ALTER TABLE agent_missions_fixed RENAME TO agent_missions;
CREATE INDEX idx_missions_status ON agent_missions (status);

COMMIT;

PRAGMA foreign_keys=on;

-- +goose Down
PRAGMA foreign_keys=off;

BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS agent_missions_old (
    id         TEXT PRIMARY KEY,
    status     TEXT NOT NULL CHECK(status IN ('PENDING', 'RUNNING', 'COMPLETED', 'FAILED', 'BURSTING')),
    payload    TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    organization_id TEXT DEFAULT 'system',
    synced_to_cloud BOOLEAN DEFAULT FALSE
);

UPDATE agent_missions SET status = 'FAILED' WHERE status = 'STUCK';

INSERT INTO agent_missions_old (id, status, payload, created_at, updated_at, organization_id, synced_to_cloud)
SELECT id, status, payload, created_at, updated_at, organization_id, synced_to_cloud FROM agent_missions;

DROP TABLE agent_missions;
ALTER TABLE agent_missions_old RENAME TO agent_missions;
CREATE INDEX idx_missions_status ON agent_missions (status);

COMMIT;

PRAGMA foreign_keys=on;
