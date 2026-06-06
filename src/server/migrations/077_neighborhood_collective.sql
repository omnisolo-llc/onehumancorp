CREATE TABLE IF NOT EXISTS collectives (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    location_center TEXT,
    radius_meters FLOAT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS collective_members (
    id TEXT PRIMARY KEY,
    collective_id TEXT NOT NULL REFERENCES collectives(id) ON DELETE CASCADE,
    tenant_id TEXT NOT NULL,
    status TEXT DEFAULT 'PENDING',
    joined_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(collective_id, tenant_id)
);
CREATE INDEX IF NOT EXISTS idx_collective_members_tenant ON collective_members(tenant_id);

CREATE TABLE IF NOT EXISTS shared_offers (
    id TEXT PRIMARY KEY,
    collective_id TEXT NOT NULL REFERENCES collectives(id) ON DELETE CASCADE,
    originating_tenant_id TEXT NOT NULL,
    target_tenant_id TEXT NOT NULL,
    discount_type TEXT NOT NULL,
    value FLOAT NOT NULL,
    auto_apply BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_shared_offers_collective ON shared_offers(collective_id);

CREATE TABLE IF NOT EXISTS collective_loyalty_balances (
    id TEXT PRIMARY KEY,
    collective_id TEXT NOT NULL REFERENCES collectives(id) ON DELETE CASCADE,
    customer_id TEXT NOT NULL,
    points_balance INTEGER DEFAULT 0,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(collective_id, customer_id)
);
CREATE INDEX IF NOT EXISTS idx_collective_loyalty_balances_customer ON collective_loyalty_balances(customer_id);

-- RLS for collectives (visible if you are a member or invited)
ALTER TABLE collectives ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_collectives ON collectives;
CREATE POLICY tenant_isolation_collectives ON collectives
    USING (
        id IN (SELECT collective_id FROM collective_members WHERE tenant_id = current_setting('app.current_tenant', true))
    )
    WITH CHECK (
        id IN (SELECT collective_id FROM collective_members WHERE tenant_id = current_setting('app.current_tenant', true))
    );

-- RLS for collective_members
ALTER TABLE collective_members ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_collective_members ON collective_members;
CREATE POLICY tenant_isolation_collective_members ON collective_members
    USING (
        tenant_id = current_setting('app.current_tenant', true) OR
        collective_id IN (SELECT collective_id FROM collective_members WHERE tenant_id = current_setting('app.current_tenant', true))
    )
    WITH CHECK (
        tenant_id = current_setting('app.current_tenant', true)
    );

-- RLS for shared_offers
ALTER TABLE shared_offers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_offers ON shared_offers;
CREATE POLICY tenant_isolation_shared_offers ON shared_offers
    USING (
        originating_tenant_id = current_setting('app.current_tenant', true) OR
        target_tenant_id = current_setting('app.current_tenant', true) OR
        collective_id IN (SELECT collective_id FROM collective_members WHERE tenant_id = current_setting('app.current_tenant', true))
    )
    WITH CHECK (
        originating_tenant_id = current_setting('app.current_tenant', true)
    );

-- RLS for collective_loyalty_balances (merchants in the collective can read/update)
ALTER TABLE collective_loyalty_balances ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_collective_loyalty_balances ON collective_loyalty_balances;
CREATE POLICY tenant_isolation_collective_loyalty_balances ON collective_loyalty_balances
    USING (
        collective_id IN (SELECT collective_id FROM collective_members WHERE tenant_id = current_setting('app.current_tenant', true))
    )
    WITH CHECK (
        collective_id IN (SELECT collective_id FROM collective_members WHERE tenant_id = current_setting('app.current_tenant', true))
    );
