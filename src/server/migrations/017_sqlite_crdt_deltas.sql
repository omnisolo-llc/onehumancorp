CREATE TABLE IF NOT EXISTS crdt_deltas (
    id TEXT PRIMARY KEY,
    entity_id TEXT NOT NULL,
    data TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    sync_status TEXT DEFAULT 'PENDING'
);

CREATE INDEX IF NOT EXISTS idx_crdt_deltas_sync_status ON crdt_deltas(sync_status);
