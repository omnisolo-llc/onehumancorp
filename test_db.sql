CREATE TABLE IF NOT EXISTS agent_missions (
    id VARCHAR PRIMARY KEY,
    status VARCHAR,
    payload TEXT,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    organization_id VARCHAR,
    mission_log TEXT
);
