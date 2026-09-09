-- +goose Up
-- Workspace slugs are reusable across organizations. Keep the relationship
-- tenant-scoped so a future MySQL/HeatWave schema can preserve the same key.

ALTER TABLE assistant_tasks
    DROP CONSTRAINT IF EXISTS assistant_tasks_workspace_id_fkey;

ALTER TABLE assistant_workspaces
    DROP CONSTRAINT IF EXISTS assistant_workspaces_pkey;

ALTER TABLE assistant_workspaces
    ADD PRIMARY KEY (tenant_id, id);

ALTER TABLE assistant_tasks
    ADD CONSTRAINT assistant_tasks_tenant_workspace_fkey
    FOREIGN KEY (tenant_id, workspace_id)
    REFERENCES assistant_workspaces(tenant_id, id)
    ON DELETE CASCADE;
