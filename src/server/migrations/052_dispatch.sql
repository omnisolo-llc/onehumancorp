CREATE TABLE IF NOT EXISTS couriers (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    type TEXT NOT NULL,
    name TEXT NOT NULL,
    phone TEXT NOT NULL,
    vehicle_type TEXT,
    status TEXT NOT NULL DEFAULT 'OFFLINE',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_couriers_organization_id ON couriers(organization_id);

ALTER TABLE couriers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_couriers ON couriers;
CREATE POLICY tenant_isolation_couriers ON couriers USING (organization_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS dispatch_sessions (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    order_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    active_courier_id TEXT REFERENCES couriers(id) ON DELETE SET NULL,
    promised_time TIMESTAMPTZ,
    delivery_fee_cents INTEGER DEFAULT 0,
    delivery_address TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_dispatch_sessions_organization_id ON dispatch_sessions(organization_id);
CREATE INDEX IF NOT EXISTS idx_dispatch_sessions_order_id ON dispatch_sessions(order_id);
CREATE INDEX IF NOT EXISTS idx_dispatch_sessions_status ON dispatch_sessions(status);

ALTER TABLE dispatch_sessions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_dispatch_sessions ON dispatch_sessions;
CREATE POLICY tenant_isolation_dispatch_sessions ON dispatch_sessions USING (organization_id::text = current_setting('app.current_tenant', true));


CREATE TABLE IF NOT EXISTS location_updates (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    dispatch_session_id TEXT NOT NULL REFERENCES dispatch_sessions(id) ON DELETE CASCADE,
    courier_id TEXT NOT NULL REFERENCES couriers(id) ON DELETE CASCADE,
    lat DOUBLE PRECISION NOT NULL,
    lng DOUBLE PRECISION NOT NULL,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_location_updates_organization_id ON location_updates(organization_id);
CREATE INDEX IF NOT EXISTS idx_location_updates_dispatch_session_id ON location_updates(dispatch_session_id);

ALTER TABLE location_updates ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_location_updates ON location_updates;
CREATE POLICY tenant_isolation_location_updates ON location_updates USING (organization_id::text = current_setting('app.current_tenant', true));
