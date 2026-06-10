CREATE TABLE IF NOT EXISTS invoices (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    client_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'DRAFT', -- DRAFT, SENT, PAID, OVERDUE
    currency TEXT NOT NULL DEFAULT 'USD',
    total_amount_cents BIGINT NOT NULL DEFAULT 0,
    due_date DATE,
    stripe_payment_link TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_invoices_tenant
ON invoices(tenant_id, status);
ALTER TABLE invoices ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_invoices ON invoices;
CREATE POLICY tenant_isolation_invoices
ON invoices
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS invoice_line_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    invoice_id TEXT NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    unit_price_cents BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_invoice_line_items_tenant_invoice
ON invoice_line_items(tenant_id, invoice_id);
ALTER TABLE invoice_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_invoice_line_items ON invoice_line_items;
CREATE POLICY tenant_isolation_invoice_line_items
ON invoice_line_items
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS expenses (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    amount_cents BIGINT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    merchant TEXT NOT NULL,
    category TEXT,
    expense_date DATE NOT NULL,
    receipt_image_url TEXT,
    project_id TEXT, -- Optional link to a specific job/project
    status TEXT NOT NULL DEFAULT 'PENDING_REVIEW', -- PENDING_REVIEW, APPROVED
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_expenses_tenant
ON expenses(tenant_id, expense_date);
ALTER TABLE expenses ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_expenses ON expenses;
CREATE POLICY tenant_isolation_expenses
ON expenses
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
