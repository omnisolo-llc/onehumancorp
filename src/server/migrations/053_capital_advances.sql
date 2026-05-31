CREATE TABLE IF NOT EXISTS capital_offers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    max_amount REAL NOT NULL,
    default_amount REAL NOT NULL,
    repayment_percentage REAL NOT NULL,
    status TEXT NOT NULL DEFAULT 'AVAILABLE',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE capital_offers ENABLE ROW LEVEL SECURITY;
CREATE POLICY capital_offers_isolation_policy ON capital_offers
    USING (tenant_id = current_setting('app.current_tenant')::text);

CREATE TABLE IF NOT EXISTS capital_advances (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    amount REAL NOT NULL,
    repayment_percentage REAL NOT NULL,
    remaining_balance REAL NOT NULL,
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE capital_advances ENABLE ROW LEVEL SECURITY;
CREATE POLICY capital_advances_isolation_policy ON capital_advances
    USING (tenant_id = current_setting('app.current_tenant')::text);

CREATE TABLE IF NOT EXISTS repayment_schedules (
    id TEXT PRIMARY KEY,
    advance_id TEXT NOT NULL,
    amount REAL NOT NULL,
    deducted_from_order_id TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
