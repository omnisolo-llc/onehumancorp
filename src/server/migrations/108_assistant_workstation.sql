CREATE TABLE IF NOT EXISTS assistant_workspaces (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    default_work_directory TEXT,
    default_model TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_assistant_workspaces_tenant_id ON assistant_workspaces(tenant_id);

CREATE TABLE IF NOT EXISTS assistant_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    workspace_id UUID NOT NULL REFERENCES assistant_workspaces(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    prompt TEXT NOT NULL,
    status TEXT NOT NULL,
    mode TEXT NOT NULL,
    model_provider_config JSONB,
    permission_profile TEXT,
    current_step TEXT,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_assistant_tasks_tenant_id ON assistant_tasks(tenant_id);
CREATE INDEX IF NOT EXISTS idx_assistant_tasks_workspace_id ON assistant_tasks(workspace_id);

CREATE TABLE IF NOT EXISTS assistant_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    task_id UUID NOT NULL REFERENCES assistant_tasks(id) ON DELETE CASCADE,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    attachments JSONB,
    tool_call_metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_assistant_messages_tenant_id ON assistant_messages(tenant_id);
CREATE INDEX IF NOT EXISTS idx_assistant_messages_task_id ON assistant_messages(task_id);

CREATE TABLE IF NOT EXISTS assistant_artifacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    task_id UUID NOT NULL REFERENCES assistant_tasks(id) ON DELETE CASCADE,
    type TEXT NOT NULL,
    filename TEXT NOT NULL,
    path TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    size BIGINT NOT NULL,
    preview_reference TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_assistant_artifacts_tenant_id ON assistant_artifacts(tenant_id);
CREATE INDEX IF NOT EXISTS idx_assistant_artifacts_task_id ON assistant_artifacts(task_id);

ALTER TABLE assistant_workspaces ENABLE ROW LEVEL SECURITY;
ALTER TABLE assistant_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE assistant_messages ENABLE ROW LEVEL SECURITY;
ALTER TABLE assistant_artifacts ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_policy_assistant_workspaces ON assistant_workspaces;
CREATE POLICY tenant_isolation_policy_assistant_workspaces ON assistant_workspaces
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
DROP POLICY IF EXISTS tenant_isolation_policy_assistant_tasks ON assistant_tasks;
CREATE POLICY tenant_isolation_policy_assistant_tasks ON assistant_tasks
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
DROP POLICY IF EXISTS tenant_isolation_policy_assistant_messages ON assistant_messages;
CREATE POLICY tenant_isolation_policy_assistant_messages ON assistant_messages
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
DROP POLICY IF EXISTS tenant_isolation_policy_assistant_artifacts ON assistant_artifacts;
CREATE POLICY tenant_isolation_policy_assistant_artifacts ON assistant_artifacts
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
