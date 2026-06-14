CREATE TABLE IF NOT EXISTS workspaces (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    default_work_dir TEXT,
    default_model TEXT,
    created_at_unix BIGINT NOT NULL,
    updated_at_unix BIGINT NOT NULL
);

ALTER TABLE workspaces ENABLE ROW LEVEL SECURITY;
CREATE POLICY workspaces_tenant_isolation ON workspaces USING (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS assistant_tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    prompt TEXT NOT NULL,
    status TEXT NOT NULL,
    mode TEXT,
    permission_profile TEXT NOT NULL,
    model_config_json JSONB,
    current_step TEXT,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at_unix BIGINT NOT NULL,
    updated_at_unix BIGINT NOT NULL
);

ALTER TABLE assistant_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY assistant_tasks_tenant_isolation ON assistant_tasks USING (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS assistant_task_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    task_id TEXT NOT NULL REFERENCES assistant_tasks(id) ON DELETE CASCADE,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    tool_metadata_json JSONB,
    created_at_unix BIGINT NOT NULL
);

ALTER TABLE assistant_task_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY assistant_task_messages_tenant_isolation ON assistant_task_messages USING (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS assistant_task_artifacts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    task_id TEXT NOT NULL REFERENCES assistant_tasks(id) ON DELETE CASCADE,
    type_ TEXT NOT NULL,
    filename TEXT NOT NULL,
    path TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    size BIGINT,
    preview_ref TEXT,
    created_at_unix BIGINT NOT NULL
);

ALTER TABLE assistant_task_artifacts ENABLE ROW LEVEL SECURITY;
CREATE POLICY assistant_task_artifacts_tenant_isolation ON assistant_task_artifacts USING (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS assistant_task_file_changes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    task_id TEXT NOT NULL REFERENCES assistant_tasks(id) ON DELETE CASCADE,
    path TEXT NOT NULL,
    change_type TEXT NOT NULL,
    summary TEXT,
    approval_status TEXT NOT NULL,
    created_at_unix BIGINT NOT NULL
);

ALTER TABLE assistant_task_file_changes ENABLE ROW LEVEL SECURITY;
CREATE POLICY assistant_task_file_changes_tenant_isolation ON assistant_task_file_changes USING (tenant_id = current_setting('app.current_tenant', true));
CREATE TABLE IF NOT EXISTS assistant_automations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    schedule TEXT NOT NULL,
    prompt TEXT NOT NULL,
    context TEXT,
    model TEXT NOT NULL,
    permission_profile TEXT NOT NULL,
    notification_channel TEXT,
    status TEXT NOT NULL,
    created_at_unix BIGINT NOT NULL,
    updated_at_unix BIGINT NOT NULL
);

ALTER TABLE assistant_automations ENABLE ROW LEVEL SECURITY;
CREATE POLICY assistant_automations_tenant_isolation ON assistant_automations USING (tenant_id = current_setting('app.current_tenant', true));
