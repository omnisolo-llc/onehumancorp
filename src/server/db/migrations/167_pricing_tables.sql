ALTER TABLE pricing_rules ADD COLUMN IF NOT EXISTS target_id TEXT;
ALTER TABLE pricing_rules ADD COLUMN IF NOT EXISTS is_active BOOLEAN DEFAULT TRUE;

ALTER TABLE pricing_rules ADD CONSTRAINT pricing_rules_target_id_key UNIQUE (tenant_id, target_id);

CREATE TABLE IF NOT EXISTS price_history (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    target_id TEXT NOT NULL,
    old_price_cents BIGINT NOT NULL,
    new_price_cents BIGINT NOT NULL,
    reason TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_price_history_tenant_target ON price_history(tenant_id, target_id);

ALTER TABLE price_history ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_price_history ON price_history;
CREATE POLICY tenant_isolation_price_history ON price_history
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
