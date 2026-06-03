-- Create ohc_expense_receipts table for local/offline queuing and storage
CREATE TABLE IF NOT EXISTS ohc_expense_receipts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    image_path TEXT,
    vendor TEXT,
    amount DECIMAL(10, 2),
    category TEXT,
    date TIMESTAMP WITH TIME ZONE,
    status TEXT NOT NULL DEFAULT 'pending', -- pending, processing, reconciled, failed
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_ohc_expense_receipts_tenant ON ohc_expense_receipts(tenant_id, created_at DESC);

-- RLS
ALTER TABLE ohc_expense_receipts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_expense_receipts ON ohc_expense_receipts;
CREATE POLICY tenant_isolation_ohc_expense_receipts
ON ohc_expense_receipts
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
