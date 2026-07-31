-- +goose Up

-- Inquiry Table
CREATE TABLE IF NOT EXISTS inquiries (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id UUID,
    source TEXT NOT NULL,
    raw_message TEXT NOT NULL,
    parsed_intent TEXT,
    urgency TEXT DEFAULT 'normal',
    status TEXT NOT NULL DEFAULT 'NEW' CHECK (status IN ('NEW', 'PROCESSING', 'QUOTED', 'CLOSED')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE inquiries ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_inquiries ON inquiries
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- QuoteDraft Table
CREATE TABLE IF NOT EXISTS quote_drafts (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    inquiry_id UUID NOT NULL REFERENCES inquiries(id) ON DELETE CASCADE,
    suggested_service TEXT NOT NULL,
    estimated_amount_cents BIGINT NOT NULL,
    suggested_time_slot TIMESTAMPTZ,
    status TEXT NOT NULL DEFAULT 'PENDING_APPROVAL' CHECK (status IN ('PENDING_APPROVAL', 'APPROVED', 'REJECTED')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE quote_drafts ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_quote_drafts ON quote_drafts
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_quote_drafts ON quote_drafts;
DROP TABLE IF EXISTS quote_drafts CASCADE;

DROP POLICY IF EXISTS tenant_isolation_inquiries ON inquiries;
DROP TABLE IF EXISTS inquiries CASCADE;
