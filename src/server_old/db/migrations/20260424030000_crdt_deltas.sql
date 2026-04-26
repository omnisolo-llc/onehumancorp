CREATE TABLE IF NOT EXISTS crdt_deltas (
    tenant_id VARCHAR NOT NULL,
    id VARCHAR NOT NULL,
    entity_id VARCHAR NOT NULL,
    data TEXT NOT NULL,
    updated_at VARCHAR NOT NULL,
    synced_to_cloud BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (tenant_id, id)
);
