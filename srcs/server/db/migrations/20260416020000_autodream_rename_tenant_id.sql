-- 058_autodream_rename_tenant_id.sql
-- Rename tenant_id to organization_id for proper tenant isolation consistency across the platform.

-- For Postgres and standard SQL we can do ALTER TABLE ... RENAME COLUMN
-- However, SQLite has historically had mixed support for RENAME COLUMN.
-- Modern SQLite supports it, so we attempt it. If not, we fall back or simply use the supported syntax.
ALTER TABLE autodream_memories_master RENAME COLUMN tenant_id TO organization_id;
ALTER TABLE ohc_memory_embeddings RENAME COLUMN tenant_id TO organization_id;

CREATE INDEX IF NOT EXISTS idx_autodream_memories_master_org ON autodream_memories_master(organization_id);
CREATE INDEX IF NOT EXISTS idx_ohc_memory_embeddings_org ON ohc_memory_embeddings(organization_id);
