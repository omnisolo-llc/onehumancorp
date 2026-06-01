CREATE TABLE IF NOT EXISTS delivery_routes (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    driver_id TEXT,
    order_ids JSONB NOT NULL DEFAULT '[]',
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS delivery_drivers (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    name TEXT NOT NULL,
    phone TEXT,
    status TEXT NOT NULL DEFAULT 'OFFLINE',
    last_lat DOUBLE PRECISION,
    last_lng DOUBLE PRECISION,
    last_location_time TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_delivery_routes_org_status ON delivery_routes(organization_id, status);
CREATE INDEX IF NOT EXISTS idx_delivery_drivers_org_status ON delivery_drivers(organization_id, status);

-- Update organizations with delivery toggle
ALTER TABLE organizations ADD COLUMN IF NOT EXISTS local_delivery_enabled BOOLEAN DEFAULT FALSE;

-- Update orders with delivery status (if not exists)
ALTER TABLE orders ADD COLUMN IF NOT EXISTS delivery_status TEXT DEFAULT 'PENDING';
ALTER TABLE orders ADD COLUMN IF NOT EXISTS delivery_driver_id TEXT;
ALTER TABLE orders ADD COLUMN IF NOT EXISTS destination_lat DOUBLE PRECISION;
ALTER TABLE orders ADD COLUMN IF NOT EXISTS destination_lng DOUBLE PRECISION;
