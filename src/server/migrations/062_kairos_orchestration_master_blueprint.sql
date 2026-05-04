CREATE TABLE IF NOT EXISTS kairos_shared_tasks (
    id VARCHAR PRIMARY KEY,
    parent_task_id VARCHAR REFERENCES kairos_shared_tasks(id),
    epic_id VARCHAR,
    tenant_id VARCHAR NOT NULL,
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    assigned_agent_id VARCHAR,
    priority VARCHAR NOT NULL DEFAULT 'P2',
    payload TEXT,
    dependencies TEXT NOT NULL DEFAULT '[]',
    locked_until TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS kairos_state_transitions (
    id VARCHAR PRIMARY KEY,
    task_id VARCHAR REFERENCES kairos_shared_tasks(id),
    tenant_id VARCHAR NOT NULL,
    from_state VARCHAR NOT NULL,
    to_state VARCHAR NOT NULL,
    agent_id VARCHAR NOT NULL,
    reason TEXT,
    occurred_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS kairos_sub_agent_jobs (
    id VARCHAR PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    organization_id VARCHAR NOT NULL,
    parent_task_id VARCHAR REFERENCES kairos_shared_tasks(id),
    payload TEXT,
    status VARCHAR NOT NULL DEFAULT 'QUEUED',
    worker_id VARCHAR,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS autodream_vector_memories (
    id VARCHAR PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    task_id VARCHAR REFERENCES kairos_shared_tasks(id),
    content TEXT NOT NULL,
    embedding TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
