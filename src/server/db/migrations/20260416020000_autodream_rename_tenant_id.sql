-- 058_autodream_rename_tenant_id.sql
-- Rename tenant_id to organization_id for proper tenant isolation consistency across the platform.

-- For Postgres and standard SQL we can do ALTER TABLE ... RENAME COLUMN
-- However, SQLite has historically had mixed support for RENAME COLUMN.
-- Modern SQLite supports it, so we attempt it. If not, we fall back or simply use the supported syntax.
CREATE TABLE IF NOT EXISTS autodream_memories_master (
    id VARCHAR PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    memory_type TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding BYTEA,
    source_task_id VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE autodream_memories_master RENAME COLUMN tenant_id TO organization_id;
CREATE TABLE IF NOT EXISTS ohc_memory_embeddings (
    id VARCHAR PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    memory_type TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding BYTEA,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    source_task_id VARCHAR
);
ALTER TABLE ohc_memory_embeddings RENAME COLUMN tenant_id TO organization_id;

CREATE INDEX idx_autodream_memories_master_org ON autodream_memories_master(organization_id);
CREATE INDEX idx_ohc_memory_embeddings_org ON ohc_memory_embeddings(organization_id);
