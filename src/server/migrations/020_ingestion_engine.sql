-- Migration 020: Autonomous cross-platform migration and ingestion engine

CREATE TABLE IF NOT EXISTS ingestion_jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL,
    status TEXT NOT NULL DEFAULT 'queued',
    source_platform TEXT NOT NULL,
    source_url TEXT NOT NULL,
    normalized_url TEXT NOT NULL,
    import_mode TEXT NOT NULL DEFAULT 'storefront_draft',
    discovered_count INT NOT NULL DEFAULT 0,
    imported_count INT NOT NULL DEFAULT 0,
    error_message TEXT,
    storefront_draft JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS ingestion_sources (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL,
    job_id UUID NOT NULL REFERENCES ingestion_jobs(id) ON DELETE CASCADE,
    platform TEXT NOT NULL,
    source_url TEXT NOT NULL,
    normalized_url TEXT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS ingestion_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL,
    job_id UUID NOT NULL REFERENCES ingestion_jobs(id) ON DELETE CASCADE,
    source_id UUID NOT NULL REFERENCES ingestion_sources(id) ON DELETE CASCADE,
    external_id TEXT,
    item_type TEXT NOT NULL DEFAULT 'product',
    title TEXT NOT NULL,
    description TEXT,
    price_cents BIGINT,
    currency TEXT DEFAULT 'USD',
    category TEXT,
    confidence_score DECIMAL NOT NULL DEFAULT 0.80,
    provenance JSONB NOT NULL DEFAULT '{}',
    raw_payload JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS ingestion_media (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL,
    job_id UUID NOT NULL REFERENCES ingestion_jobs(id) ON DELETE CASCADE,
    item_id UUID REFERENCES ingestion_items(id) ON DELETE CASCADE,
    source_url TEXT NOT NULL,
    media_type TEXT NOT NULL DEFAULT 'image',
    alt_text TEXT,
    provenance JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ingestion_jobs_tenant_created ON ingestion_jobs(tenant_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_ingestion_items_job ON ingestion_items(job_id);
CREATE INDEX IF NOT EXISTS idx_ingestion_media_job ON ingestion_media(job_id);

ALTER TABLE ingestion_jobs ENABLE ROW LEVEL SECURITY;
ALTER TABLE ingestion_jobs FORCE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ingestion_jobs ON ingestion_jobs;
CREATE POLICY tenant_isolation_ingestion_jobs ON ingestion_jobs USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE ingestion_sources ENABLE ROW LEVEL SECURITY;
ALTER TABLE ingestion_sources FORCE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ingestion_sources ON ingestion_sources;
CREATE POLICY tenant_isolation_ingestion_sources ON ingestion_sources USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE ingestion_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE ingestion_items FORCE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ingestion_items ON ingestion_items;
CREATE POLICY tenant_isolation_ingestion_items ON ingestion_items USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE ingestion_media ENABLE ROW LEVEL SECURITY;
ALTER TABLE ingestion_media FORCE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ingestion_media ON ingestion_media;
CREATE POLICY tenant_isolation_ingestion_media ON ingestion_media USING (tenant_id::text = current_setting('app.current_tenant', true));
