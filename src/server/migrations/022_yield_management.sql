-- Migration 022: Autonomous Dynamic Pricing & Yield Management Engine

CREATE TABLE IF NOT EXISTS yield_strategies (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    target_entity_id TEXT NOT NULL,
    target_entity_type TEXT NOT NULL, -- 'inventory_item' or 'booking_slot'
    predicted_spoilage_risk FLOAT NOT NULL,
    expiration_window TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS dynamic_price_adjustments (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    yield_strategy_id TEXT NOT NULL REFERENCES yield_strategies(id) ON DELETE CASCADE,
    original_price DECIMAL NOT NULL,
    adjusted_price DECIMAL NOT NULL,
    marketing_draft_copy TEXT NOT NULL,
    approval_status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'approved', 'rejected'
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Enable RLS
ALTER TABLE yield_strategies ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_yield_strategies ON yield_strategies USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE dynamic_price_adjustments ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_dynamic_price_adjustments ON dynamic_price_adjustments USING (tenant_id::text = current_setting('app.current_tenant', true));
