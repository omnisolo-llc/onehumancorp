CREATE TABLE IF NOT EXISTS inventory_reservations (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    product_id VARCHAR NOT NULL,
    quantity INTEGER NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_inventory_reservations_tenant ON inventory_reservations (tenant_id);
CREATE INDEX IF NOT EXISTS idx_inventory_reservations_product ON inventory_reservations (product_id);
CREATE INDEX IF NOT EXISTS idx_inventory_reservations_expires_at ON inventory_reservations (expires_at);

ALTER TABLE inventory_reservations ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_policy ON inventory_reservations
    USING (tenant_id = current_setting('app.current_tenant_id', true));

-- To track POS devices sync status
CREATE TABLE IF NOT EXISTS pos_devices (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    device_name VARCHAR NOT NULL,
    last_sync_at TIMESTAMPTZ,
    status VARCHAR DEFAULT 'active',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE pos_devices ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_policy_pos_devices ON pos_devices
    USING (tenant_id = current_setting('app.current_tenant_id', true));
