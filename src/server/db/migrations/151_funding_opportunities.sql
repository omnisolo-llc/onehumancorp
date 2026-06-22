-- +goose Up
-- Universal Autonomous Grant and Funding Engine

CREATE TABLE IF NOT EXISTS funding_opportunities (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    grant_name TEXT NOT NULL,
    amount BIGINT NOT NULL,
    draft_proposal_text TEXT,
    status TEXT NOT NULL DEFAULT 'Drafted', -- 'Drafted', 'Submitted', 'Won', 'Rejected'
    deadline TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('funding_opportunities') IS NOT NULL THEN
        ALTER TABLE funding_opportunities ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_funding_opportunities ON funding_opportunities USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_funding_opportunities ON funding_opportunities;
END
$$;

DROP TABLE IF EXISTS funding_opportunities CASCADE;
