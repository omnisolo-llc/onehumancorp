CREATE TABLE IF NOT EXISTS capital_contracts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    advance_amount DOUBLE PRECISION NOT NULL,
    flat_fee DOUBLE PRECISION NOT NULL,
    repayment_percentage DOUBLE PRECISION NOT NULL,
    repaid_amount DOUBLE PRECISION NOT NULL DEFAULT 0,
    status VARCHAR(50) NOT NULL DEFAULT 'Offered',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_capital_contracts_tenant ON capital_contracts(tenant_id);

CREATE TABLE IF NOT EXISTS capital_offers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    advance_amount DOUBLE PRECISION NOT NULL,
    flat_fee DOUBLE PRECISION NOT NULL,
    repayment_percentage DOUBLE PRECISION NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'Offered',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_capital_offers_tenant ON capital_offers(tenant_id);

-- Simulate intercepting revenue: we need a trigger or logic on revenue to update repaid_amount.
-- For now, we will track this via application logic in service.rs when handling ledger events.
