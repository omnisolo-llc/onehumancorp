-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS dependencies JSONB NOT NULL DEFAULT '[]';

CREATE TABLE IF NOT EXISTS state_machine_transitions (
    id VARCHAR PRIMARY KEY,
    entity_id VARCHAR NOT NULL,
    entity_type VARCHAR NOT NULL,
    from_state VARCHAR NOT NULL,
    to_state VARCHAR NOT NULL,
    agent_id VARCHAR,
    reason TEXT,
    occurred_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS state_machine_transitions;
ALTER TABLE shared_tasks DROP COLUMN IF EXISTS dependencies;
-- +goose StatementEnd
