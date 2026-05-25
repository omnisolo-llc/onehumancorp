-- Migration: 013_kairos_shared_tasks.sql
-- Renames tenant_id to organization_id on shared_tasks and recreates RLS policy to match organization_id

ALTER TABLE shared_tasks RENAME COLUMN tenant_id TO organization_id;

-- Drop the old policy and create a new one based on the new column name
DROP POLICY IF EXISTS tenant_isolation_shared_tasks ON shared_tasks;
CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (organization_id::text = current_setting('app.current_tenant', true));
