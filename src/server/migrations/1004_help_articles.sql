-- Keep the PostgreSQL schema used by SQLx in sync with the help API and the
-- launch-readiness seed endpoint. The original table definition only existed
-- in the legacy src/server/db/migrations tree, which run_migrations does not use.

CREATE TABLE IF NOT EXISTS help_articles (
    id BIGSERIAL PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    category TEXT NOT NULL,
    title TEXT NOT NULL,
    desc_text TEXT NOT NULL,
    link TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_help_articles_tenant_id
    ON help_articles (tenant_id);

ALTER TABLE help_articles ENABLE ROW LEVEL SECURITY;
ALTER TABLE help_articles FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_help_articles ON help_articles;
CREATE POLICY tenant_isolation_help_articles ON help_articles
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS video_tutorials (
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    id INTEGER NOT NULL,
    title TEXT NOT NULL,
    duration TEXT NOT NULL,
    video_url TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, id)
);

CREATE INDEX IF NOT EXISTS idx_video_tutorials_tenant_id
    ON video_tutorials (tenant_id);

ALTER TABLE video_tutorials ENABLE ROW LEVEL SECURITY;
ALTER TABLE video_tutorials FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_video_tutorials ON video_tutorials;
CREATE POLICY tenant_isolation_video_tutorials ON video_tutorials
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS tooltips (
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    id TEXT NOT NULL,
    text TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, id)
);

ALTER TABLE tooltips ENABLE ROW LEVEL SECURITY;
ALTER TABLE tooltips FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_tooltips ON tooltips;
CREATE POLICY tenant_isolation_tooltips ON tooltips
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS walkthrough_steps (
    id BIGSERIAL PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    page TEXT NOT NULL,
    step_order INTEGER NOT NULL,
    selector TEXT NOT NULL,
    title TEXT NOT NULL,
    text TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_walkthrough_steps_tenant_page_order
    ON walkthrough_steps (tenant_id, page, step_order);

ALTER TABLE walkthrough_steps ENABLE ROW LEVEL SECURITY;
ALTER TABLE walkthrough_steps FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_walkthrough_steps ON walkthrough_steps;
CREATE POLICY tenant_isolation_walkthrough_steps ON walkthrough_steps
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
