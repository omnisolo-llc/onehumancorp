
-- Create invoices table
CREATE TABLE IF NOT EXISTS invoices (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    merchant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    status TEXT NOT NULL,
    total_amount DOUBLE PRECISION NOT NULL,
    currency TEXT NOT NULL,
    due_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS on invoices
ALTER TABLE invoices ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_invoices ON invoices USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Create invoice_line_items table
CREATE TABLE IF NOT EXISTS invoice_line_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    invoice_id TEXT NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS on invoice_line_items
ALTER TABLE invoice_line_items ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_invoice_line_items ON invoice_line_items USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Create payment_events table
CREATE TABLE IF NOT EXISTS payment_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    invoice_id TEXT NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    type TEXT NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS on payment_events
ALTER TABLE payment_events ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_payment_events ON payment_events USING (tenant_id::text = current_setting('app.current_tenant', true));
