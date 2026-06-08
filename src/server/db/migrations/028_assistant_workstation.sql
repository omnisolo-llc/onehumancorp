-- +goose Up
-- Migration 028: Add assistant workstation tables

CREATE TABLE IF NOT EXISTS assistant_workspaces (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    default_work_dir TEXT,
    default_model TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS assistant_tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES assistant_workspaces(id),
    title TEXT NOT NULL,
    prompt TEXT NOT NULL,
    status TEXT NOT NULL,
    mode TEXT,
    permission_profile TEXT NOT NULL,
    model_config JSONB,
    current_step TEXT,
    archived BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS assistant_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    task_id TEXT NOT NULL REFERENCES assistant_tasks(id),
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    tool_metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS assistant_artifacts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    task_id TEXT NOT NULL REFERENCES assistant_tasks(id),
    type TEXT NOT NULL,
    filename TEXT NOT NULL,
    path TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    size BIGINT,
    preview_ref TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS assistant_file_changes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    task_id TEXT NOT NULL REFERENCES assistant_tasks(id),
    path TEXT NOT NULL,
    change_type TEXT NOT NULL,
    summary TEXT,
    approval_status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('assistant_workspaces') IS NOT NULL THEN
        ALTER TABLE assistant_workspaces ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_assistant_workspaces ON assistant_workspaces USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
    IF to_regclass('assistant_tasks') IS NOT NULL THEN
        ALTER TABLE assistant_tasks ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_assistant_tasks ON assistant_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
    IF to_regclass('assistant_messages') IS NOT NULL THEN
        ALTER TABLE assistant_messages ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_assistant_messages ON assistant_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
    IF to_regclass('assistant_artifacts') IS NOT NULL THEN
        ALTER TABLE assistant_artifacts ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_assistant_artifacts ON assistant_artifacts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
    IF to_regclass('assistant_file_changes') IS NOT NULL THEN
        ALTER TABLE assistant_file_changes ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_assistant_file_changes ON assistant_file_changes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_assistant_workspaces ON assistant_workspaces;
    DROP POLICY IF EXISTS tenant_isolation_assistant_tasks ON assistant_tasks;
    DROP POLICY IF EXISTS tenant_isolation_assistant_messages ON assistant_messages;
    DROP POLICY IF EXISTS tenant_isolation_assistant_artifacts ON assistant_artifacts;
    DROP POLICY IF EXISTS tenant_isolation_assistant_file_changes ON assistant_file_changes;
END
$$;

DROP TABLE IF EXISTS assistant_file_changes CASCADE;
DROP TABLE IF EXISTS assistant_artifacts CASCADE;
DROP TABLE IF EXISTS assistant_messages CASCADE;
DROP TABLE IF EXISTS assistant_tasks CASCADE;
DROP TABLE IF EXISTS assistant_workspaces CASCADE;
