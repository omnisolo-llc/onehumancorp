CREATE TABLE IF NOT EXISTS voice_agent_config (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    is_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    primary_language TEXT NOT NULL DEFAULT 'English',
    custom_instructions TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id)
);
-- Multi-tenant Row Level Security (RLS)
ALTER TABLE voice_agent_config ENABLE ROW LEVEL SECURITY;

CREATE POLICY "tenant_isolation_voice_config" ON voice_agent_config
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true));

CREATE TABLE IF NOT EXISTS call_logs (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    caller_phone TEXT NOT NULL,
    status TEXT NOT NULL,
    duration_seconds INTEGER,
    summary TEXT,
    transcript TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
-- Multi-tenant Row Level Security (RLS)
ALTER TABLE call_logs ENABLE ROW LEVEL SECURITY;

CREATE POLICY "tenant_isolation_call_logs" ON call_logs
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true));
