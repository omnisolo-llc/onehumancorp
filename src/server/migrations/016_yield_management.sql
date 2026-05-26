-- Migration 016: Yield Management Engine

CREATE TABLE IF NOT EXISTS yield_profiles (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    target_id TEXT NOT NULL, -- references a product or service
    target_type TEXT NOT NULL, -- 'product' or 'service'
    enabled BOOLEAN NOT NULL DEFAULT true,
    min_price_cents BIGINT NOT NULL,
    max_price_cents BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE yield_profiles ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_yield_profiles ON yield_profiles USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS price_adjustment_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    yield_profile_id TEXT NOT NULL REFERENCES yield_profiles(id) ON DELETE CASCADE,
    old_price_cents BIGINT NOT NULL,
    new_price_cents BIGINT NOT NULL,
    reason TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE price_adjustment_events ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_price_adjustment_events ON price_adjustment_events USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS capacity_states (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    yield_profile_id TEXT NOT NULL REFERENCES yield_profiles(id) ON DELETE CASCADE,
    available BIGINT NOT NULL,
    total BIGINT NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE capacity_states ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_capacity_states ON capacity_states USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS demand_signals (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    yield_profile_id TEXT NOT NULL REFERENCES yield_profiles(id) ON DELETE CASCADE,
    signal_type TEXT NOT NULL,
    score FLOAT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE demand_signals ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_demand_signals ON demand_signals USING (tenant_id::text = current_setting('app.current_tenant', true));
