CREATE TABLE IF NOT EXISTS physical_touchpoints (
    touchpoint_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    hash_id TEXT NOT NULL UNIQUE,
    qr_image_url TEXT,
    entity_type TEXT NOT NULL, -- e.g., 'product', 'invoice', 'service'
    entity_id TEXT NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_physical_touchpoints_tenant_id ON physical_touchpoints(tenant_id);
CREATE INDEX IF NOT EXISTS idx_physical_touchpoints_entity ON physical_touchpoints(entity_type, entity_id);

CREATE TABLE IF NOT EXISTS scan_events (
    scan_id TEXT PRIMARY KEY,
    touchpoint_id TEXT NOT NULL REFERENCES physical_touchpoints(touchpoint_id) ON DELETE CASCADE,
    scanned_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    buyer_device_hash TEXT
);

CREATE INDEX IF NOT EXISTS idx_scan_events_touchpoint_id ON scan_events(touchpoint_id);
