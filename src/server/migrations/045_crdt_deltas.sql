-- 045_crdt_deltas.sql
CREATE TABLE IF NOT EXISTS crdt_deltas (
    tenant_id TEXT NOT NULL,
    id TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    data TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    synced_to_cloud BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (tenant_id, id)
);

CREATE TABLE IF NOT EXISTS local_mcp_rag_tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    payload TEXT NOT NULL,
    escalation_status TEXT NOT NULL
);
