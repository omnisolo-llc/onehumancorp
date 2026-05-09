CREATE TABLE IF NOT EXISTS agent_missions (
    id TEXT PRIMARY KEY,
    status TEXT NOT NULL,
    payload TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    organization_id TEXT,
    mission_log TEXT
);

INSERT INTO agent_missions (id, status, payload, mission_log) VALUES ('m_handoff', 'blocked', '{"task":"drain"}', 'I cannot finish an OHC product mission. Handover required.');

UPDATE agent_missions
SET status = 'blocked',
    mission_log = mission_log || '
' || 'I cannot finish an OHC product mission. Queue is empty or resources are missing.'
WHERE id = 'm_handoff';
