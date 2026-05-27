-- Migration 017: Core Data Model Architecture

CREATE TABLE IF NOT EXISTS offerings (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    type TEXT NOT NULL, -- "physical", "service", "digital"
    name TEXT NOT NULL,
    description TEXT,
    price DECIMAL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT REFERENCES customers(id) ON DELETE CASCADE,
    status TEXT DEFAULT 'pending', -- "pending", "paid", "completed"
    total_amount DECIMAL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS transaction_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    transaction_id TEXT REFERENCES transactions(id) ON DELETE CASCADE,
    offering_id TEXT REFERENCES offerings(id) ON DELETE CASCADE,
    quantity INT DEFAULT 1,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE offerings ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_offerings ON offerings USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE transactions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_transactions ON transactions USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE transaction_items ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_transaction_items ON transaction_items USING (tenant_id::text = current_setting('app.current_tenant', true));
