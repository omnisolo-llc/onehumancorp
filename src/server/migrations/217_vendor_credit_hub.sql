-- OHC Vendor Credit Hub Schema

CREATE TABLE IF NOT EXISTS credit_facilities (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    approved_limit_usd DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    utilized_amount_usd DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    dynamic_score DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    underwriter_version VARCHAR(255),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS vendor_relations (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    vendor_name VARCHAR(255) NOT NULL,
    vendor_email VARCHAR(255) NOT NULL,
    current_terms VARCHAR(255) NOT NULL DEFAULT 'COD', -- COD | NET_15 | NET_30 | NET_60
    term_status VARCHAR(255) NOT NULL DEFAULT 'APPROVED', -- APPROVED | NEGOTIATING | DENIED
    terms_granted_at TIMESTAMP WITH TIME ZONE
);

CREATE TABLE IF NOT EXISTS supplier_invoices (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    vendor_relation_id UUID NOT NULL REFERENCES vendor_relations(id) ON DELETE CASCADE,
    invoice_number VARCHAR(255) NOT NULL,
    total_amount DOUBLE PRECISION NOT NULL,
    currency VARCHAR(255) NOT NULL DEFAULT 'USD',
    due_date TIMESTAMP WITH TIME ZONE,
    status VARCHAR(255) NOT NULL DEFAULT 'UNPAID' -- UNPAID | SWEEPING | PAID | OVERDUE
);

CREATE TABLE IF NOT EXISTS factoring_discounts (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    client_invoice_id VARCHAR(255) NOT NULL,
    invoice_amount DOUBLE PRECISION NOT NULL,
    advance_rate DOUBLE PRECISION NOT NULL DEFAULT 0.85,
    flat_fee_pct DOUBLE PRECISION NOT NULL DEFAULT 0.02,
    advanced_amount_usd DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    factoring_status VARCHAR(255) NOT NULL DEFAULT 'APPLIED', -- APPLIED | DISBURSED | REPAID
    disbursed_at TIMESTAMP WITH TIME ZONE
);

CREATE TABLE IF NOT EXISTS ledger_sweep_configs (
    id UUID PRIMARY KEY,
    supplier_invoice_id UUID NOT NULL REFERENCES supplier_invoices(id) ON DELETE CASCADE,
    daily_sweep_pct DOUBLE PRECISION NOT NULL DEFAULT 0.10,
    maximum_sweep_usd DOUBLE PRECISION,
    accumulated_sweep_usd DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    last_sweep_run TIMESTAMP WITH TIME ZONE
);

-- Enable RLS for newly created tables
ALTER TABLE credit_facilities ENABLE ROW LEVEL SECURITY;
ALTER TABLE credit_facilities FORCE ROW LEVEL SECURITY;

ALTER TABLE vendor_relations ENABLE ROW LEVEL SECURITY;
ALTER TABLE vendor_relations FORCE ROW LEVEL SECURITY;

ALTER TABLE supplier_invoices ENABLE ROW LEVEL SECURITY;
ALTER TABLE supplier_invoices FORCE ROW LEVEL SECURITY;

ALTER TABLE factoring_discounts ENABLE ROW LEVEL SECURITY;
ALTER TABLE factoring_discounts FORCE ROW LEVEL SECURITY;

-- Create Policies for RLS
CREATE POLICY tenant_isolation_credit_facilities ON credit_facilities
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE POLICY tenant_isolation_vendor_relations ON vendor_relations
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE POLICY tenant_isolation_supplier_invoices ON supplier_invoices
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE POLICY tenant_isolation_factoring_discounts ON factoring_discounts
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
