CREATE TABLE IF NOT EXISTS inquiries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    source TEXT NOT NULL,
    raw_message TEXT NOT NULL,
    parsed_intent TEXT,
    urgency TEXT DEFAULT 'normal',
    status TEXT NOT NULL DEFAULT 'NEW' CHECK (status IN ('NEW', 'PROCESSING', 'QUOTED', 'CLOSED')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE inquiries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inquiries ON inquiries;
CREATE POLICY tenant_isolation_inquiries ON inquiries
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
