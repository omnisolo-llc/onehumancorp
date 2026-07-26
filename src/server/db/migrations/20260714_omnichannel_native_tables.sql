CREATE TABLE IF NOT EXISTS unified_threads (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    channel TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE unified_threads ENABLE ROW LEVEL SECURITY;
CREATE POLICY unified_threads_tenant_isolation_policy ON unified_threads FOR ALL USING (tenant_id = current_setting('app.current_tenant', true)::text);

CREATE TABLE IF NOT EXISTS unified_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    thread_id TEXT NOT NULL REFERENCES unified_threads(id),
    sender_type TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE unified_messages ENABLE ROW LEVEL SECURITY;
CREATE POLICY unified_messages_tenant_isolation_policy ON unified_messages FOR ALL USING (
    EXISTS (
        SELECT 1 FROM unified_threads WHERE unified_threads.id = unified_messages.thread_id AND unified_threads.tenant_id = current_setting('app.current_tenant', true)::text
    )
);

CREATE TABLE IF NOT EXISTS unified_triage_actions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    thread_id TEXT NOT NULL REFERENCES unified_threads(id),
    action_type TEXT NOT NULL,
    action_payload TEXT,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE unified_triage_actions ENABLE ROW LEVEL SECURITY;
CREATE POLICY unified_triage_actions_tenant_isolation_policy ON unified_triage_actions FOR ALL USING (
    EXISTS (
        SELECT 1 FROM unified_threads WHERE unified_threads.id = unified_triage_actions.thread_id AND unified_threads.tenant_id = current_setting('app.current_tenant', true)::text
    )
);
