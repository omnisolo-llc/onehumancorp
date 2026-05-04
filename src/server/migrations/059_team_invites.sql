CREATE TABLE IF NOT EXISTS team_invites (
    id VARCHAR PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    team_id VARCHAR NOT NULL,
    inviter_id VARCHAR NOT NULL,
    invitee_id VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE team_invites ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can only access their own tenant's team invites"
    ON team_invites
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true));
