-- +goose Up
-- Migration 028: Add Assistant Jarvis Workstation tables

CREATE TABLE IF NOT EXISTS assistant_workspaces (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    default_model TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS assistant_tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES assistant_workspaces(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    prompt TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    mode TEXT,
    model TEXT,
    permission_profile TEXT,
    current_step TEXT,
    archived BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS assistant_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    task_id TEXT NOT NULL REFERENCES assistant_tasks(id) ON DELETE CASCADE,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS assistant_artifacts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    task_id TEXT NOT NULL REFERENCES assistant_tasks(id) ON DELETE CASCADE,
    type TEXT NOT NULL,
    filename TEXT NOT NULL,
    path_ref TEXT NOT NULL,
    mime_type TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_assistant_tasks_workspace_id ON assistant_tasks(workspace_id);
CREATE INDEX IF NOT EXISTS idx_assistant_messages_task_id ON assistant_messages(task_id);
CREATE INDEX IF NOT EXISTS idx_assistant_artifacts_task_id ON assistant_artifacts(task_id);

DO $$
BEGIN
    IF to_regclass('assistant_workspaces') IS NOT NULL THEN
        ALTER TABLE assistant_workspaces ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies WHERE schemaname = current_schema() AND tablename = 'assistant_workspaces' AND policyname = 'tenant_isolation_assistant_workspaces'
        ) THEN
            CREATE POLICY tenant_isolation_assistant_workspaces ON assistant_workspaces USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    IF to_regclass('assistant_tasks') IS NOT NULL THEN
        ALTER TABLE assistant_tasks ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies WHERE schemaname = current_schema() AND tablename = 'assistant_tasks' AND policyname = 'tenant_isolation_assistant_tasks'
        ) THEN
            CREATE POLICY tenant_isolation_assistant_tasks ON assistant_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    IF to_regclass('assistant_messages') IS NOT NULL THEN
        ALTER TABLE assistant_messages ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies WHERE schemaname = current_schema() AND tablename = 'assistant_messages' AND policyname = 'tenant_isolation_assistant_messages'
        ) THEN
            CREATE POLICY tenant_isolation_assistant_messages ON assistant_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    IF to_regclass('assistant_artifacts') IS NOT NULL THEN
        ALTER TABLE assistant_artifacts ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies WHERE schemaname = current_schema() AND tablename = 'assistant_artifacts' AND policyname = 'tenant_isolation_assistant_artifacts'
        ) THEN
            CREATE POLICY tenant_isolation_assistant_artifacts ON assistant_artifacts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('assistant_workspaces') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_assistant_workspaces ON assistant_workspaces;
        ALTER TABLE assistant_workspaces DISABLE ROW LEVEL SECURITY;
    END IF;

    IF to_regclass('assistant_tasks') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_assistant_tasks ON assistant_tasks;
        ALTER TABLE assistant_tasks DISABLE ROW LEVEL SECURITY;
    END IF;

    IF to_regclass('assistant_messages') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_assistant_messages ON assistant_messages;
        ALTER TABLE assistant_messages DISABLE ROW LEVEL SECURITY;
    END IF;

    IF to_regclass('assistant_artifacts') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_assistant_artifacts ON assistant_artifacts;
        ALTER TABLE assistant_artifacts DISABLE ROW LEVEL SECURITY;
    END IF;
END
$$;

DROP TABLE IF EXISTS assistant_artifacts CASCADE;
DROP TABLE IF EXISTS assistant_messages CASCADE;
DROP TABLE IF EXISTS assistant_tasks CASCADE;
DROP TABLE IF EXISTS assistant_workspaces CASCADE;
