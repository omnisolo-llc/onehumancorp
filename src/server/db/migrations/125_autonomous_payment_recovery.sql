-- Autonomous Payment Recovery & Invoice Management Schema Expansion

-- Ensure enum values are handled safely, or use string types if Postgres version lacks enum migration tools
-- We'll use text with check constraints to allow easy expansion and adhere to the existing conventions

-- Modify invoices table to support enhanced tracking for autonomous recovery
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS payment_status TEXT DEFAULT 'draft';
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS view_count INTEGER NOT NULL DEFAULT 0;
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS amount_paid_cents INTEGER NOT NULL DEFAULT 0;

-- Ensure total_amount_cents exists (there was a total_amount double precision, let's add cents for precision)
ALTER TABLE invoices ADD COLUMN IF NOT EXISTS total_amount_cents INTEGER NOT NULL DEFAULT 0;

-- Update the status check to include our new states
-- (Using text to avoid enum complexities, relying on app logic or check constraints)

-- Create Communication Events table to track reminders
CREATE TABLE IF NOT EXISTS invoice_communication_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    invoice_id TEXT NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'drafted', -- drafted, approved, sent
    channel TEXT NOT NULL DEFAULT 'email', -- email, sms, whatsapp
    drafted_content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_invoice_communication_events_invoice_id ON invoice_communication_events(invoice_id);
CREATE INDEX IF NOT EXISTS idx_invoice_communication_events_tenant_id ON invoice_communication_events(tenant_id);

ALTER TABLE invoice_communication_events ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_invoice_communication_events ON invoice_communication_events;
CREATE POLICY tenant_isolation_invoice_communication_events
ON invoice_communication_events
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
