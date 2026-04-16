CREATE TABLE IF NOT EXISTS missions (
    id UUID PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    priority VARCHAR(50) CHECK (priority IN ('P0', 'P1', 'P2', 'P3')),
    status VARCHAR(50) CHECK (status IN ('PENDING', 'IN_PROGRESS', 'BLOCKED', 'DONE')),
    agent_assigned VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS agent_state (
    agent_id VARCHAR(255) PRIMARY KEY,
    current_mission_id UUID REFERENCES missions(id),
    status VARCHAR(50) CHECK (status IN ('IDLE', 'WORKING', 'ERROR')),
    lock_id VARCHAR(255),
    lock_expires_at TIMESTAMP WITH TIME ZONE,
    last_heartbeat TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
