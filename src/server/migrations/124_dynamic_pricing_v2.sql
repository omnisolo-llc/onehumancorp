-- Migration 124: Dynamic Pricing V2 - Price History and Enhanced Rules

-- Add target_id and is_active to pricing_rules
ALTER TABLE pricing_rules ADD COLUMN IF NOT EXISTS target_id TEXT;
ALTER TABLE pricing_rules ADD COLUMN IF NOT EXISTS is_active BOOLEAN DEFAULT TRUE;

CREATE INDEX IF NOT EXISTS idx_pricing_rules_target ON pricing_rules(target_id);

-- Create price_history table
CREATE TABLE IF NOT EXISTS price_history (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    target_id TEXT NOT NULL,
    rule_id UUID REFERENCES pricing_rules(id) ON DELETE SET NULL,
    old_price_cents BIGINT NOT NULL,
    new_price_cents BIGINT NOT NULL,
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_price_history_tenant_target ON price_history(tenant_id, target_id);

-- Enable RLS for price_history
ALTER TABLE price_history ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_price_history ON price_history;
CREATE POLICY tenant_isolation_price_history ON price_history
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
