-- A receipt proves a manual inventory mutation, not an order or provider action.
CREATE TABLE IF NOT EXISTS inventory_adjustment_receipts (
    tenant_id TEXT NOT NULL,
    client_mutation_id TEXT NOT NULL,
    item_id TEXT NOT NULL,
    request_identity TEXT NOT NULL,
    receipt_json TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, client_mutation_id)
);
CREATE INDEX IF NOT EXISTS inventory_adjustment_receipts_product
    ON inventory_adjustment_receipts (tenant_id, item_id);
ALTER TABLE inventory_adjustment_receipts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS inventory_adjustment_receipts_tenant ON inventory_adjustment_receipts;
CREATE POLICY inventory_adjustment_receipts_tenant ON inventory_adjustment_receipts
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
