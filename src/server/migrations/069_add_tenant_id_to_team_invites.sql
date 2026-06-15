-- Add tenant_id to team_invites to fix aggregation bug
ALTER TABLE team_invites ADD COLUMN IF NOT EXISTS tenant_id TEXT;
UPDATE team_invites SET tenant_id = team_id WHERE tenant_id IS NULL;
ALTER TABLE team_invites ALTER COLUMN tenant_id SET NOT NULL;

CREATE INDEX IF NOT EXISTS idx_team_invites_tenant_id ON team_invites(tenant_id);

DROP POLICY IF EXISTS tenant_isolation_team_invites ON team_invites;
CREATE POLICY tenant_isolation_team_invites ON team_invites USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
