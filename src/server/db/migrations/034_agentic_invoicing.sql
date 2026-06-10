-- Agentic Invoicing System Migration

CREATE TABLE IF NOT EXISTS invoices (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    client_id TEXT NOT NULL,
    client_name TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    due_date BIGINT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    total_amount DOUBLE PRECISION NOT NULL,
    stripe_invoice_id TEXT,
    stripe_payment_link TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_invoices_tenant_id ON invoices(tenant_id);

ALTER TABLE invoices ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_invoices ON invoices;
CREATE POLICY tenant_isolation_invoices
ON invoices
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS invoice_line_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    invoice_id TEXT NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price DOUBLE PRECISION NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_invoice_line_items_invoice_id ON invoice_line_items(invoice_id);
CREATE INDEX IF NOT EXISTS idx_invoice_line_items_tenant_id ON invoice_line_items(tenant_id);

ALTER TABLE invoice_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_invoice_line_items ON invoice_line_items;
CREATE POLICY tenant_isolation_invoice_line_items
ON invoice_line_items
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
