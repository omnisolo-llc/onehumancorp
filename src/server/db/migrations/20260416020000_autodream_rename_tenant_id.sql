-- 058_autodream_rename_tenant_id.sql
-- In adherence with new guidelines: use tenant_id instead of organization_id.

CREATE TABLE IF NOT EXISTS autodream_memories_master (
    id VARCHAR PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    memory_type TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding BYTEA,
    source_task_id VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
-- Remove renaming to organization_id and enable RLS
ALTER TABLE autodream_memories_master ENABLE ROW LEVEL SECURITY;

CREATE TABLE IF NOT EXISTS ohc_memory_embeddings (
    id VARCHAR PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    memory_type TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding BYTEA,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    source_task_id VARCHAR
);
-- Remove renaming to organization_id and enable RLS
ALTER TABLE ohc_memory_embeddings ENABLE ROW LEVEL SECURITY;

CREATE INDEX idx_autodream_memories_master_org ON autodream_memories_master(tenant_id);
CREATE INDEX idx_ohc_memory_embeddings_org ON ohc_memory_embeddings(tenant_id);
