-- +goose Up
DO $$
BEGIN
    -- help_articles
    IF EXISTS (SELECT 1 FROM pg_tables WHERE schemaname = 'public' AND tablename = 'help_articles') THEN
        ALTER TABLE help_articles ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies WHERE schemaname = current_schema() AND tablename = 'help_articles' AND policyname = 'tenant_isolation_help_articles'
        ) THEN
            CREATE POLICY tenant_isolation_help_articles ON help_articles USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    -- video_tutorials
    IF EXISTS (SELECT 1 FROM pg_tables WHERE schemaname = 'public' AND tablename = 'video_tutorials') THEN
        ALTER TABLE video_tutorials ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies WHERE schemaname = current_schema() AND tablename = 'video_tutorials' AND policyname = 'tenant_isolation_video_tutorials'
        ) THEN
            CREATE POLICY tenant_isolation_video_tutorials ON video_tutorials USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    -- tooltips
    IF EXISTS (SELECT 1 FROM pg_tables WHERE schemaname = 'public' AND tablename = 'tooltips') THEN
        ALTER TABLE tooltips ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies WHERE schemaname = current_schema() AND tablename = 'tooltips' AND policyname = 'tenant_isolation_tooltips'
        ) THEN
            CREATE POLICY tenant_isolation_tooltips ON tooltips USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;

    -- walkthrough_steps
    IF EXISTS (SELECT 1 FROM pg_tables WHERE schemaname = 'public' AND tablename = 'walkthrough_steps') THEN
        ALTER TABLE walkthrough_steps ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies WHERE schemaname = current_schema() AND tablename = 'walkthrough_steps' AND policyname = 'tenant_isolation_walkthrough_steps'
        ) THEN
            CREATE POLICY tenant_isolation_walkthrough_steps ON walkthrough_steps USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END $$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_help_articles ON help_articles;
    DROP POLICY IF EXISTS tenant_isolation_video_tutorials ON video_tutorials;
    DROP POLICY IF EXISTS tenant_isolation_tooltips ON tooltips;
    DROP POLICY IF EXISTS tenant_isolation_walkthrough_steps ON walkthrough_steps;
END $$;
