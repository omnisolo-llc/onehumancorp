CREATE TABLE IF NOT EXISTS intake_messages (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_name TEXT,
    channel TEXT NOT NULL,
    message TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending_triage',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS booking_drafts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    intake_message_id TEXT NOT NULL REFERENCES intake_messages(id) ON DELETE CASCADE,
    scheduled_for TIMESTAMPTZ,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS payment_links (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    intake_message_id TEXT NOT NULL REFERENCES intake_messages(id) ON DELETE CASCADE,
    amount NUMERIC(10, 2),
    currency TEXT DEFAULT 'USD',
    link_url TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS proposed_tasks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    intake_message_id TEXT NOT NULL REFERENCES intake_messages(id) ON DELETE CASCADE,
    booking_draft_id TEXT REFERENCES booking_drafts(id) ON DELETE CASCADE,
    payment_link_id TEXT REFERENCES payment_links(id) ON DELETE CASCADE,
    draft_reply TEXT,
    status TEXT NOT NULL DEFAULT 'proposed',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE intake_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_intake_messages ON intake_messages;
CREATE POLICY tenant_isolation_intake_messages ON intake_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE booking_drafts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_booking_drafts ON booking_drafts;
CREATE POLICY tenant_isolation_booking_drafts ON booking_drafts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE payment_links ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_payment_links ON payment_links;
CREATE POLICY tenant_isolation_payment_links ON payment_links USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE proposed_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_proposed_tasks ON proposed_tasks;
CREATE POLICY tenant_isolation_proposed_tasks ON proposed_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
