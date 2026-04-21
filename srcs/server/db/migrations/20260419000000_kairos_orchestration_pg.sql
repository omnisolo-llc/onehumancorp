-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS swarm_tasks (
    id VARCHAR PRIMARY KEY,
    mission_id VARCHAR NOT NULL,
    parent_plan_id VARCHAR,
    dependencies JSONB NOT NULL DEFAULT '[]',
    title VARCHAR NOT NULL,
    description TEXT,
    priority VARCHAR DEFAULT 'P2',
    status VARCHAR NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'ASSIGNED', 'IN_PROGRESS', 'REVIEW', 'COMPLETED', 'FAILED')),
    assigned_agent_id VARCHAR,
    locked_until TIMESTAMPTZ,
    payload JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS state_machine_transitions (
    id VARCHAR PRIMARY KEY,
    entity_id VARCHAR NOT NULL,
    entity_type VARCHAR NOT NULL,
    from_state VARCHAR NOT NULL,
    to_state VARCHAR NOT NULL,
    agent_id VARCHAR,
    reason TEXT,
    occurred_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_sm_entity ON state_machine_transitions(entity_id, entity_type);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS state_machine_transitions;
DROP TABLE IF EXISTS swarm_tasks;
-- +goose StatementEnd
