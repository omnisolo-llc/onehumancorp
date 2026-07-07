-- +goose Up
CREATE TABLE IF NOT EXISTS conversational_intake_sessions (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id UUID,
    inbox_thread_id TEXT,
    status TEXT NOT NULL CHECK (status IN ('GATHERING_INFO', 'DRAFTING_QUOTE', 'PENDING_APPROVAL', 'QUOTE_SENT', 'CLOSED')),
    collected_data JSONB DEFAULT '{}',
    quote_id UUID REFERENCES quotes(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE conversational_intake_sessions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_conversational_intake_sessions
    ON conversational_intake_sessions
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_conversational_intake_sessions ON conversational_intake_sessions;
DROP TABLE IF EXISTS conversational_intake_sessions CASCADE;
