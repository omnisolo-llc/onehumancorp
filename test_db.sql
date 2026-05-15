CREATE TABLE IF NOT EXISTS agent_missions (
    id TEXT PRIMARY KEY,
    status TEXT NOT NULL,
    payload TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    organization_id TEXT,
    mission_log TEXT
);

INSERT INTO agent_missions (id, status, payload, mission_log) VALUES ('m_handoff', 'blocked', '{"task":"drain"}', 'I cannot finish an OHC product mission. Handover required.
Blockers:
- The user prompt provides only role and protocol definitions.
- No specific issue, bug, feature request, or concrete task to implement was described.
- As an Implementer, I require a defined mission to execute.');

UPDATE agent_missions
SET status = 'blocked',
    mission_log = mission_log || '
- The user prompt provides only role and protocol definitions.
- No specific issue, bug, feature request, or concrete task to implement was described.
- As an Implementer, I require a defined mission to execute.'
WHERE id = 'm_handoff';
CREATE TABLE IF NOT EXISTS distributed_locks (
    key VARCHAR(255) PRIMARY KEY,
    owner VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);
