-- +goose Up
-- Update the SQLite schema to add a status enum including 'BURSTING'.
-- Since SQLite doesn't support ENUMs directly or ALTER TABLE for CHECK constraints easily,
-- we implement this via recreating the table with the CHECK constraint.

PRAGMA foreign_keys=off;

CREATE TABLE IF NOT EXISTS agent_missions_new (
    id         TEXT PRIMARY KEY,
    status     TEXT NOT NULL CHECK(status IN ('PENDING', 'RUNNING', 'COMPLETED', 'FAILED', 'BURSTING')),
    payload    TEXT NOT NULL,
    organization_id TEXT,
    synced_to_cloud BOOLEAN DEFAULT FALSE,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO agent_missions_new (id, status, payload, created_at)
SELECT id, status, payload, created_at FROM agent_missions;

DROP TABLE agent_missions;
ALTER TABLE agent_missions_new RENAME TO agent_missions;
CREATE INDEX idx_missions_status ON agent_missions (status);

PRAGMA foreign_keys=on;

-- +goose Down
PRAGMA foreign_keys=off;

CREATE TABLE IF NOT EXISTS agent_missions_old (
    id         TEXT PRIMARY KEY,
    status     TEXT NOT NULL CHECK(status IN ('PENDING', 'RUNNING', 'COMPLETED', 'FAILED')),
    payload    TEXT NOT NULL,
    organization_id TEXT,
    synced_to_cloud BOOLEAN DEFAULT FALSE,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

UPDATE agent_missions SET status = 'PENDING' WHERE status = 'BURSTING';

INSERT INTO agent_missions_old (id, status, payload, created_at)
SELECT id, status, payload, created_at FROM agent_missions;

DROP TABLE agent_missions;
ALTER TABLE agent_missions_old RENAME TO agent_missions;
CREATE INDEX idx_missions_status ON agent_missions (status);

PRAGMA foreign_keys=on;
