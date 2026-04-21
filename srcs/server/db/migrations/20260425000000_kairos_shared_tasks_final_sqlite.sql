-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN dependencies TEXT NOT NULL DEFAULT '[]';

CREATE TABLE IF NOT EXISTS state_machine_transitions (
    id TEXT PRIMARY KEY,
    entity_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    from_state TEXT NOT NULL,
    to_state TEXT NOT NULL,
    agent_id TEXT,
    reason TEXT,
    occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS state_machine_transitions;
ALTER TABLE shared_tasks DROP COLUMN dependencies;
-- +goose StatementEnd
