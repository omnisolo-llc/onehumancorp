-- Migration 056: Consistently use organization_id for core entity tables

-- 1. Rename columns
ALTER TABLE users RENAME COLUMN tenant_id TO organization_id;
ALTER TABLE roles RENAME COLUMN tenant_id TO organization_id;
ALTER TABLE tasks RENAME COLUMN tenant_id TO organization_id;
ALTER TABLE agent_inbox RENAME COLUMN tenant_id TO organization_id;
ALTER TABLE meeting_rooms RENAME COLUMN tenant_id TO organization_id;

-- 2. Drop and recreate constraints on users
ALTER TABLE users DROP CONSTRAINT IF EXISTS users_username_tenant_id_key;
ALTER TABLE users DROP CONSTRAINT IF EXISTS users_email_tenant_id_key;
ALTER TABLE users DROP CONSTRAINT IF EXISTS users_oidc_subject_tenant_id_key;
ALTER TABLE users ADD CONSTRAINT users_username_organization_id_key UNIQUE (username, organization_id);
ALTER TABLE users ADD CONSTRAINT users_email_organization_id_key UNIQUE (email, organization_id);
ALTER TABLE users ADD CONSTRAINT users_oidc_subject_organization_id_key UNIQUE (oidc_subject, organization_id);

-- 3. Drop and recreate constraints on roles
ALTER TABLE roles DROP CONSTRAINT IF EXISTS roles_name_tenant_id_key;
ALTER TABLE roles ADD CONSTRAINT roles_name_organization_id_key UNIQUE (name, organization_id);

-- 4. Drop old policies
DROP POLICY IF EXISTS tenant_isolation_users ON users;
DROP POLICY IF EXISTS tenant_isolation_roles ON roles;
DROP POLICY IF EXISTS tenant_isolation_tasks ON tasks;
DROP POLICY IF EXISTS tenant_isolation_agent_inbox ON agent_inbox;
DROP POLICY IF EXISTS tenant_isolation_meeting_rooms ON meeting_rooms;

-- 5. Recreate policies with organization_id
CREATE POLICY tenant_isolation_users ON users USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_roles ON roles USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_tasks ON tasks USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_agent_inbox ON agent_inbox USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_meeting_rooms ON meeting_rooms USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

-- 6. Update indexes
DROP INDEX IF EXISTS idx_tasks_tenant_id;
CREATE INDEX IF NOT EXISTS idx_tasks_organization_id ON tasks(organization_id);

-- Also update policies on meeting_transcripts to use organization_id for the meeting_rooms subquery
DROP POLICY IF EXISTS tenant_isolation_meeting_transcripts ON meeting_transcripts;
CREATE POLICY tenant_isolation_meeting_transcripts
    ON meeting_transcripts
    USING (meeting_id IN (SELECT id FROM meeting_rooms WHERE organization_id::text = current_setting('app.current_tenant', true)))
    WITH CHECK (meeting_id IN (SELECT id FROM meeting_rooms WHERE organization_id::text = current_setting('app.current_tenant', true)));
