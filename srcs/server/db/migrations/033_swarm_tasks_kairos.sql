-- +goose Up
-- +goose StatementBegin
ALTER TABLE swarm_tasks ADD COLUMN organization_id TEXT;
UPDATE swarm_tasks SET organization_id = 'default' WHERE organization_id IS NULL;

CREATE TABLE IF NOT EXISTS state_machine_transitions (
    id TEXT PRIMARY KEY,
    entity_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    from_state TEXT NOT NULL,
    to_state TEXT NOT NULL,
    agent_id TEXT,
    reason TEXT,
    occurred_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_sm_entity ON state_machine_transitions(entity_id, entity_type);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE swarm_tasks DROP COLUMN IF EXISTS organization_id;
-- +goose StatementEnd
