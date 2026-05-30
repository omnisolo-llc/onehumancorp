-- Migration 022: Invisible Catalog
CREATE TABLE IF NOT EXISTS product_video_scans (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    video_url TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS draft_catalog_items (
    id TEXT PRIMARY KEY,
    scan_id TEXT REFERENCES product_video_scans(id) ON DELETE CASCADE,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    estimated_price_cents BIGINT,
    image_url TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING_REVIEW',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE product_video_scans ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_video_scans ON product_video_scans USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE draft_catalog_items ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_draft_items ON draft_catalog_items USING (tenant_id::text = current_setting('app.current_tenant', true));
