-- Migration: 013_kairos_orchestration.sql
-- Robust schema for KAIROS Orchestration

-- Ensure shared_tasks has organization_id for tenant isolation as requested
ALTER TABLE shared_tasks ADD COLUMN organization_id TEXT;

-- Create a robust task_dependencies mapping table for shared_tasks
-- This replaces/complements the JSONB dependencies column for better relational queries
CREATE TABLE IF NOT EXISTS shared_task_dependencies (
    task_id TEXT NOT NULL,
    depends_on_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (task_id, depends_on_id)
);

-- Indexing for fast dependency graph traversal
CREATE INDEX IF NOT EXISTS idx_shared_task_deps_org ON shared_task_dependencies(organization_id);
CREATE INDEX IF NOT EXISTS idx_shared_task_deps_depends ON shared_task_dependencies(depends_on_id);

-- Ensure consolidated_memory has organization_id and proper vector support
ALTER TABLE consolidated_memory ADD COLUMN organization_id TEXT;

-- Enable RLS for the new mapping table (Postgres only)
-- Note: SQLite will ignore these or fail if not handled by the migrator.
-- The Rust migrator for Postgres will handle this.
ALTER TABLE shared_task_dependencies ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_shared_task_dependencies ON shared_task_dependencies
USING (organization_id::text = current_setting('app.current_tenant', true));

-- Add organization_id index to shared_tasks for performance
CREATE INDEX IF NOT EXISTS idx_shared_tasks_org_id ON shared_tasks(organization_id);
