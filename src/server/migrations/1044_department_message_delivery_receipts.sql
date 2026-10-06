CREATE TABLE IF NOT EXISTS integration_credentials (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    integration_id TEXT NOT NULL,
    bot_token TEXT,
    api_token TEXT,
    from_phone TEXT,
    chat_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE IF EXISTS integration_credentials ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = current_schema()
          AND tablename = 'integration_credentials'
          AND policyname = 'tenant_isolation_integration_credentials'
    ) THEN
        CREATE POLICY tenant_isolation_integration_credentials ON integration_credentials
            USING (tenant_id::text = current_setting('app.current_tenant', true))
            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END $$;

-- One-shot provider attempt fence. Acceptance is not delivery.
CREATE TABLE IF NOT EXISTS department_message_dispatches (
    tenant_id TEXT NOT NULL,
    action_id TEXT NOT NULL,
    inbox_message_id TEXT NOT NULL,
    inbox_kind TEXT NOT NULL CHECK (inbox_kind IN ('inbox','omni')),
    payload_hash TEXT NOT NULL,
    provider_binding TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('unknown','accepted','rejected','blocked')),
    provider_message_id TEXT,
    detail TEXT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    PRIMARY KEY (tenant_id,action_id),
    CHECK ((state='accepted' AND provider_message_id IS NOT NULL AND length(provider_message_id)>0)
        OR (state<>'accepted' AND provider_message_id IS NULL))
);
-- Proven no-effect failures can receive a new explicit review. An ambiguous
-- or accepted attempt permanently fences this inbox identity against resends.
CREATE UNIQUE INDEX IF NOT EXISTS department_message_inbox_attempt_fence
ON department_message_dispatches(tenant_id,inbox_message_id)
WHERE state IN ('unknown','accepted');
ALTER TABLE department_message_dispatches ENABLE ROW LEVEL SECURITY;
ALTER TABLE department_message_dispatches FORCE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_department_message_dispatches ON department_message_dispatches
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
