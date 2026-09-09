-- +goose Up
-- The assistant APIs are shared by every database backend. PostgreSQL had RLS
-- migrations for these tables, but no migration that created the tables.

CREATE TABLE IF NOT EXISTS assistant_workspaces (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    default_work_dir TEXT,
    default_model TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS assistant_tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    workspace_id TEXT NOT NULL REFERENCES assistant_workspaces(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    prompt TEXT NOT NULL,
    status TEXT NOT NULL,
    mode TEXT,
    permission_profile TEXT NOT NULL,
    model_config JSONB,
    current_step TEXT,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS assistant_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    task_id TEXT NOT NULL REFERENCES assistant_tasks(id) ON DELETE CASCADE,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    tool_metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS assistant_artifacts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    task_id TEXT NOT NULL REFERENCES assistant_tasks(id) ON DELETE CASCADE,
    type TEXT NOT NULL,
    filename TEXT NOT NULL,
    path TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    size BIGINT,
    preview_ref TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS assistant_file_changes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    task_id TEXT NOT NULL REFERENCES assistant_tasks(id) ON DELETE CASCADE,
    path TEXT NOT NULL,
    change_type TEXT NOT NULL,
    summary TEXT,
    approval_status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS assistant_memory_records (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    scope TEXT NOT NULL DEFAULT 'global',
    source TEXT,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS assistant_skills (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    category TEXT NOT NULL DEFAULT 'Custom',
    source TEXT NOT NULL DEFAULT 'database',
    status TEXT NOT NULL,
    version TEXT,
    description TEXT,
    config JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, name)
);

CREATE TABLE IF NOT EXISTS assistant_connectors (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    kind TEXT NOT NULL DEFAULT 'custom',
    status TEXT NOT NULL,
    oauth BOOLEAN NOT NULL DEFAULT FALSE,
    config JSONB,
    last_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, name)
);

DO $$
DECLARE
    table_name TEXT;
    policy_name TEXT;
BEGIN
    FOREACH table_name IN ARRAY ARRAY[
        'assistant_workspaces', 'assistant_tasks', 'assistant_messages',
        'assistant_artifacts', 'assistant_file_changes', 'assistant_memory_records',
        'assistant_skills', 'assistant_connectors'
    ]
    LOOP
        policy_name := format('tenant_isolation_%s', table_name);
        EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', table_name);
        EXECUTE format('ALTER TABLE %I FORCE ROW LEVEL SECURITY', table_name);
        EXECUTE format('DROP POLICY IF EXISTS %I ON %I', policy_name, table_name);
        EXECUTE format(
            'CREATE POLICY %I ON %I USING (tenant_id = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id = current_setting(''app.current_tenant'', true))',
            policy_name,
            table_name
        );
    END LOOP;
END
$$;

GRANT ALL PRIVILEGES ON
    assistant_workspaces, assistant_tasks, assistant_messages,
    assistant_artifacts, assistant_file_changes, assistant_memory_records,
    assistant_skills, assistant_connectors
TO ohc_bypassrls;
