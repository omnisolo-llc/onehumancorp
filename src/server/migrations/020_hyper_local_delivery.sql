CREATE TABLE IF NOT EXISTS delivery_batches (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    status TEXT DEFAULT 'pending',
    scheduled_for TIMESTAMPTZ,
    optimized_route_data JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE delivery_batches ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_delivery_batches ON delivery_batches USING (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS delivery_stops (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    batch_id TEXT NOT NULL REFERENCES delivery_batches(id) ON DELETE CASCADE,
    order_id TEXT NOT NULL,
    sequence_index INTEGER NOT NULL,
    status TEXT DEFAULT 'pending',
    proof_of_delivery JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE delivery_stops ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_delivery_stops ON delivery_stops USING (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS driver_sessions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    batch_id TEXT NOT NULL REFERENCES delivery_batches(id) ON DELETE CASCADE,
    phone_number TEXT NOT NULL,
    magic_link_token TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE driver_sessions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_driver_sessions ON driver_sessions USING (tenant_id = current_setting('app.current_tenant', true));

CREATE INDEX IF NOT EXISTS idx_driver_sessions_magic_link_token ON driver_sessions (magic_link_token);
