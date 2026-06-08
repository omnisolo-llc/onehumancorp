-- Assistant Workstation Core Tables

CREATE TABLE IF NOT EXISTS assistant_workspaces (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    default_work_directory TEXT,
    default_model TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_assistant_workspaces_tenant_id ON assistant_workspaces(tenant_id);

ALTER TABLE assistant_workspaces ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_workspaces ON assistant_workspaces;
CREATE POLICY tenant_isolation_assistant_workspaces ON assistant_workspaces
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS assistant_tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES assistant_workspaces(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    prompt TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    mode TEXT,
    model TEXT,
    provider TEXT,
    permission_profile TEXT DEFAULT 'Guarded',
    current_step TEXT,
    archived BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_assistant_tasks_tenant_id ON assistant_tasks(tenant_id);
CREATE INDEX IF NOT EXISTS idx_assistant_tasks_workspace_id ON assistant_tasks(workspace_id);

ALTER TABLE assistant_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_tasks ON assistant_tasks;
CREATE POLICY tenant_isolation_assistant_tasks ON assistant_tasks
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS assistant_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    task_id TEXT NOT NULL REFERENCES assistant_tasks(id) ON DELETE CASCADE,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    attachments JSONB DEFAULT '[]',
    tool_calls JSONB DEFAULT '[]',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_assistant_messages_tenant_id ON assistant_messages(tenant_id);
CREATE INDEX IF NOT EXISTS idx_assistant_messages_task_id ON assistant_messages(task_id);

ALTER TABLE assistant_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_messages ON assistant_messages;
CREATE POLICY tenant_isolation_assistant_messages ON assistant_messages
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS assistant_artifacts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    task_id TEXT NOT NULL REFERENCES assistant_tasks(id) ON DELETE CASCADE,
    type TEXT NOT NULL,
    filename TEXT NOT NULL,
    path TEXT,
    mime_type TEXT,
    size BIGINT,
    preview TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_assistant_artifacts_tenant_id ON assistant_artifacts(tenant_id);
CREATE INDEX IF NOT EXISTS idx_assistant_artifacts_task_id ON assistant_artifacts(task_id);

ALTER TABLE assistant_artifacts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_artifacts ON assistant_artifacts;
CREATE POLICY tenant_isolation_assistant_artifacts ON assistant_artifacts
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS assistant_file_changes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    task_id TEXT NOT NULL REFERENCES assistant_tasks(id) ON DELETE CASCADE,
    path TEXT NOT NULL,
    change_type TEXT,
    diff TEXT,
    summary TEXT,
    approval_status TEXT DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_assistant_file_changes_tenant_id ON assistant_file_changes(tenant_id);
CREATE INDEX IF NOT EXISTS idx_assistant_file_changes_task_id ON assistant_file_changes(task_id);

ALTER TABLE assistant_file_changes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_file_changes ON assistant_file_changes;
CREATE POLICY tenant_isolation_assistant_file_changes ON assistant_file_changes
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS assistant_approvals (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    task_id TEXT NOT NULL REFERENCES assistant_tasks(id) ON DELETE CASCADE,
    tool_name TEXT NOT NULL,
    args JSONB,
    status TEXT DEFAULT 'pending',
    risk_level TEXT DEFAULT 'high',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_assistant_approvals_tenant_id ON assistant_approvals(tenant_id);
CREATE INDEX IF NOT EXISTS idx_assistant_approvals_task_id ON assistant_approvals(task_id);

ALTER TABLE assistant_approvals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_approvals ON assistant_approvals;
CREATE POLICY tenant_isolation_assistant_approvals ON assistant_approvals
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
