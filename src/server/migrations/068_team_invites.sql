-- Create team_invites table for viral loop growth

CREATE TABLE IF NOT EXISTS team_invites (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    team_id TEXT NOT NULL,
    inviter_id TEXT NOT NULL,
    invitee_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_team_invites_tenant_id ON team_invites(tenant_id);
CREATE INDEX IF NOT EXISTS idx_team_invites_team_id ON team_invites(team_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_team_invites_inviter_id ON team_invites(inviter_id);
CREATE INDEX IF NOT EXISTS idx_team_invites_invitee_id ON team_invites(invitee_id);

ALTER TABLE team_invites ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_team_invites ON team_invites;
CREATE POLICY tenant_isolation_team_invites ON team_invites USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
