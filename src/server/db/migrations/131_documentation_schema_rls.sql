-- +goose Up
-- Migration 131: Add RLS to documentation schema tables

ALTER TABLE help_articles ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_help_articles ON help_articles
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE video_tutorials ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_video_tutorials ON video_tutorials
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE tooltips ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_tooltips ON tooltips
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE walkthrough_steps ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_walkthrough_steps ON walkthrough_steps
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_help_articles ON help_articles;
ALTER TABLE help_articles DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_video_tutorials ON video_tutorials;
ALTER TABLE video_tutorials DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_tooltips ON tooltips;
ALTER TABLE tooltips DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_walkthrough_steps ON walkthrough_steps;
ALTER TABLE walkthrough_steps DISABLE ROW LEVEL SECURITY;
