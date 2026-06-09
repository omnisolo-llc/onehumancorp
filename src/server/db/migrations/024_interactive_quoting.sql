-- Migration: Agentic Interactive Quoting & Deposit Engine

-- Quote Table
CREATE TABLE IF NOT EXISTS quotes (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id UUID NOT NULL, -- Assuming customers table exists, otherwise might need TEXT/foreign key logic
    status TEXT NOT NULL CHECK (status IN ('DRAFT', 'PENDING_APPROVAL', 'SENT', 'ACCEPTED', 'REJECTED', 'EXPIRED')),
    valid_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Quote Line Items Table
CREATE TABLE IF NOT EXISTS quote_line_items (
    id UUID PRIMARY KEY,
    quote_id UUID NOT NULL REFERENCES quotes(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    unit_price_cents BIGINT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    is_optional BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Pricing Heuristic Table
CREATE TABLE IF NOT EXISTS pricing_heuristics (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    service_category TEXT NOT NULL,
    base_rate_cents BIGINT NOT NULL,
    materials_markup_percentage NUMERIC NOT NULL,
    instructions TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Enable RLS and setup tenant isolation policies
ALTER TABLE quotes ENABLE ROW LEVEL SECURITY;
ALTER TABLE quote_line_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE pricing_heuristics ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_quotes ON quotes USING (tenant_id = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_quote_line_items ON quote_line_items USING (
    quote_id IN (SELECT id FROM quotes WHERE tenant_id = current_setting('app.current_tenant', true))
);
CREATE POLICY tenant_isolation_pricing_heuristics ON pricing_heuristics USING (tenant_id = current_setting('app.current_tenant', true));
