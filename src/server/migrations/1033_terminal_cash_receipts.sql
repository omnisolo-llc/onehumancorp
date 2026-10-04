-- A cash sale is acknowledged only after these receipts, its order, and every
-- stock deduction commit together. No external provider wait is involved.
CREATE TABLE IF NOT EXISTS terminal_cash_receipts (
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    operation_id TEXT NOT NULL,
    order_id TEXT NOT NULL REFERENCES orders(id),
    amount_cents BIGINT NOT NULL CHECK (amount_cents >= 0),
    customer_id TEXT REFERENCES customers(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, operation_id),
    UNIQUE (order_id)
);
CREATE TABLE IF NOT EXISTS terminal_cash_receipt_items (
    tenant_id TEXT NOT NULL,
    operation_id TEXT NOT NULL,
    product_id TEXT NOT NULL REFERENCES products(id),
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    amount_cents BIGINT NOT NULL CHECK (amount_cents >= 0),
    lock_id TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (tenant_id, operation_id, product_id),
    FOREIGN KEY (tenant_id, operation_id) REFERENCES terminal_cash_receipts(tenant_id, operation_id) ON DELETE CASCADE
);
-- Older callers may supply a prior reservation. It cannot be sold twice, even
-- when a different operation ID is used after the original response is lost.
CREATE UNIQUE INDEX terminal_cash_reservation_once
    ON terminal_cash_receipt_items(tenant_id, product_id, lock_id) WHERE lock_id <> '';
ALTER TABLE terminal_cash_receipts ENABLE ROW LEVEL SECURITY;
ALTER TABLE terminal_cash_receipts FORCE ROW LEVEL SECURITY;
CREATE POLICY terminal_cash_receipts_tenant ON terminal_cash_receipts
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
ALTER TABLE terminal_cash_receipt_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE terminal_cash_receipt_items FORCE ROW LEVEL SECURITY;
CREATE POLICY terminal_cash_receipt_items_tenant ON terminal_cash_receipt_items
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
