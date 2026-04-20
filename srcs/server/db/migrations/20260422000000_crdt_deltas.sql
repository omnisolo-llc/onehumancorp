-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS crdt_deltas (
    id VARCHAR(255) PRIMARY KEY,
    entity_id VARCHAR(255) NOT NULL,
    data TEXT NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    synced_to_cloud BOOLEAN DEFAULT FALSE
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS crdt_deltas;
-- +goose StatementEnd
