-- Create assistant_workspaces table
CREATE TABLE IF NOT EXISTS assistant_workspaces (
    tenant_id TEXT NOT NULL,
    id TEXT NOT NULL,
    name TEXT NOT NULL,
    default_work_directory TEXT,
    default_model TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, id)
);

CREATE INDEX IF NOT EXISTS idx_assistant_workspaces_tenant ON assistant_workspaces(tenant_id);

ALTER TABLE assistant_workspaces ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_workspaces ON assistant_workspaces;
CREATE POLICY tenant_isolation_assistant_workspaces ON assistant_workspaces USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


-- Create assistant_tasks table
CREATE TABLE IF NOT EXISTS assistant_tasks (
    tenant_id TEXT NOT NULL,
    id TEXT NOT NULL,
    workspace_id TEXT NOT NULL,
    title TEXT NOT NULL,
    prompt TEXT NOT NULL,
    status TEXT NOT NULL,
    mode TEXT NOT NULL,
    model TEXT NOT NULL,
    provider TEXT NOT NULL,
    permission_profile TEXT NOT NULL,
    current_step TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, id),
    FOREIGN KEY (tenant_id, workspace_id) REFERENCES assistant_workspaces(tenant_id, id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_assistant_tasks_tenant ON assistant_tasks(tenant_id);
CREATE INDEX IF NOT EXISTS idx_assistant_tasks_workspace ON assistant_tasks(tenant_id, workspace_id);

ALTER TABLE assistant_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_tasks ON assistant_tasks;
CREATE POLICY tenant_isolation_assistant_tasks ON assistant_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


-- Create assistant_messages table
CREATE TABLE IF NOT EXISTS assistant_messages (
    tenant_id TEXT NOT NULL,
    id TEXT NOT NULL,
    task_id TEXT NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    attachments JSONB DEFAULT '[]'::jsonb,
    tool_call_metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, id),
    FOREIGN KEY (tenant_id, task_id) REFERENCES assistant_tasks(tenant_id, id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_assistant_messages_tenant ON assistant_messages(tenant_id);
CREATE INDEX IF NOT EXISTS idx_assistant_messages_task ON assistant_messages(tenant_id, task_id);

ALTER TABLE assistant_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_messages ON assistant_messages;
CREATE POLICY tenant_isolation_assistant_messages ON assistant_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


-- Create assistant_artifacts table
CREATE TABLE IF NOT EXISTS assistant_artifacts (
    tenant_id TEXT NOT NULL,
    id TEXT NOT NULL,
    task_id TEXT NOT NULL,
    type TEXT NOT NULL,
    filename TEXT NOT NULL,
    path TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    size BIGINT NOT NULL,
    preview_ref TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, id),
    FOREIGN KEY (tenant_id, task_id) REFERENCES assistant_tasks(tenant_id, id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_assistant_artifacts_tenant ON assistant_artifacts(tenant_id);
CREATE INDEX IF NOT EXISTS idx_assistant_artifacts_task ON assistant_artifacts(tenant_id, task_id);

ALTER TABLE assistant_artifacts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_artifacts ON assistant_artifacts;
CREATE POLICY tenant_isolation_assistant_artifacts ON assistant_artifacts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


-- Create assistant_changes table
CREATE TABLE IF NOT EXISTS assistant_changes (
    tenant_id TEXT NOT NULL,
    id TEXT NOT NULL,
    task_id TEXT NOT NULL,
    path TEXT NOT NULL,
    change_type TEXT NOT NULL,
    diff TEXT NOT NULL,
    approval_status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, id),
    FOREIGN KEY (tenant_id, task_id) REFERENCES assistant_tasks(tenant_id, id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_assistant_changes_tenant ON assistant_changes(tenant_id);
CREATE INDEX IF NOT EXISTS idx_assistant_changes_task ON assistant_changes(tenant_id, task_id);

ALTER TABLE assistant_changes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_changes ON assistant_changes;
CREATE POLICY tenant_isolation_assistant_changes ON assistant_changes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
