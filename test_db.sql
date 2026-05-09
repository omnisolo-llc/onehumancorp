CREATE TABLE IF NOT EXISTS swarm_tasks (
    mission_id UUID PRIMARY KEY,
    parent_plan_id TEXT,
    dependencies JSONB,
    title VARCHAR,
    status VARCHAR,
    assigned_agent_id VARCHAR,
    payload JSONB,
    locked_until TIMESTAMP,
    created_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS state_machine_transitions (
    id SERIAL PRIMARY KEY,
    entity_id UUID,
    entity_type VARCHAR,
    from_state VARCHAR,
    to_state VARCHAR,
    agent_id VARCHAR,
    reason TEXT,
    created_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS consolidated_memory (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    agent_id TEXT,
    content TEXT NOT NULL,
    embedding BLOB,
    source_type TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_referenced_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    reference_count INTEGER DEFAULT 0,
    reliability_score INTEGER DEFAULT 50,
    owner_override BOOLEAN DEFAULT FALSE,
    metadata TEXT
);
