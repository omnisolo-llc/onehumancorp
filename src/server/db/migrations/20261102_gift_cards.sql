-- Create Gift Cards Table
CREATE TABLE IF NOT EXISTS gift_cards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    customer_id UUID,
    initial_balance NUMERIC(15, 2) NOT NULL,
    current_balance NUMERIC(15, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE,
    qr_payload TEXT NOT NULL,
    CONSTRAINT fk_tenant FOREIGN KEY (tenant_id) REFERENCES tenants(id)
);

-- Enable Row Level Security
ALTER TABLE gift_cards ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Tenant Isolation Policy" ON gift_cards
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id')::uuid);

-- Create Ledger Transactions Table
CREATE TABLE IF NOT EXISTS ledger_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    gift_card_id UUID NOT NULL REFERENCES gift_cards(id),
    amount NUMERIC(15, 2) NOT NULL,
    transaction_type VARCHAR(50) NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    offline_synced BOOLEAN DEFAULT FALSE,
    CONSTRAINT fk_tenant_ledger FOREIGN KEY (tenant_id) REFERENCES tenants(id)
);

-- Enable Row Level Security
ALTER TABLE ledger_transactions ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Tenant Isolation Policy" ON ledger_transactions
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id')::uuid);
