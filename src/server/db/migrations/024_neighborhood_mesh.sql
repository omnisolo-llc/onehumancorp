
CREATE TABLE IF NOT EXISTS ohc_collective (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    location_center TEXT,
    radius_meters FLOAT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS ohc_collective_member (
    collective_id TEXT NOT NULL REFERENCES ohc_collective(id) ON DELETE CASCADE,
    tenant_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT PENDING,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (collective_id, tenant_id)
);

CREATE INDEX IF NOT EXISTS idx_collective_member_tenant ON ohc_collective_member(tenant_id);
ALTER TABLE ohc_collective_member ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_collective_member ON ohc_collective_member;
CREATE POLICY tenant_isolation_ohc_collective_member
ON ohc_collective_member
USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS ohc_shared_offer (
    id TEXT PRIMARY KEY,
    collective_id TEXT NOT NULL REFERENCES ohc_collective(id) ON DELETE CASCADE,
    originating_tenant_id TEXT NOT NULL,
    target_tenant_id TEXT NOT NULL,
    discount_type TEXT NOT NULL,
    value FLOAT NOT NULL,
    auto_apply BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_shared_offer_collective ON ohc_shared_offer(collective_id);
ALTER TABLE ohc_shared_offer ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_shared_offer ON ohc_shared_offer;
CREATE POLICY tenant_isolation_ohc_shared_offer
ON ohc_shared_offer
USING (originating_tenant_id = current_setting('app.current_tenant', true) OR target_tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS ohc_collective_loyalty_balance (
    collective_id TEXT NOT NULL REFERENCES ohc_collective(id) ON DELETE CASCADE,
    buyer_id TEXT NOT NULL,
    balance INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (collective_id, buyer_id)
);
