CREATE TABLE IF NOT EXISTS crdt_deltas (
    id TEXT PRIMARY KEY,
    entity_id TEXT NOT NULL,
    data TEXT NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    synced_to_cloud BOOLEAN DEFAULT false
);

CREATE INDEX IF NOT EXISTS idx_crdt_deltas_entity_id ON crdt_deltas(entity_id);
