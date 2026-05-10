-- +goose Up
-- +goose StatementBegin
-- +goose postgres
ALTER TABLE kairos_sub_agent_jobs ADD COLUMN IF NOT EXISTS tenant_id UUID NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';
ALTER TABLE kairos_state_transitions ADD COLUMN IF NOT EXISTS tenant_id UUID NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000';

ALTER TABLE kairos_sub_agent_jobs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_kairos_sub_agent_jobs ON kairos_sub_agent_jobs;
CREATE POLICY tenant_isolation_kairos_sub_agent_jobs ON kairos_sub_agent_jobs
    USING (tenant_id = nullif(current_setting('app.current_tenant', true), '')::uuid);

ALTER TABLE kairos_state_transitions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_kairos_state_transitions ON kairos_state_transitions;
CREATE POLICY tenant_isolation_kairos_state_transitions ON kairos_state_transitions
    USING (tenant_id = nullif(current_setting('app.current_tenant', true), '')::uuid);
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose sqlite3
ALTER TABLE kairos_sub_agent_jobs ADD COLUMN tenant_id TEXT NOT NULL DEFAULT 'system';
ALTER TABLE kairos_state_transitions ADD COLUMN tenant_id TEXT NOT NULL DEFAULT 'system';
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- +goose postgres
DROP POLICY IF EXISTS tenant_isolation_kairos_state_transitions ON kairos_state_transitions;
ALTER TABLE kairos_state_transitions DISABLE ROW LEVEL SECURITY;
ALTER TABLE kairos_state_transitions DROP COLUMN IF EXISTS tenant_id;

DROP POLICY IF EXISTS tenant_isolation_kairos_sub_agent_jobs ON kairos_sub_agent_jobs;
ALTER TABLE kairos_sub_agent_jobs DISABLE ROW LEVEL SECURITY;
ALTER TABLE kairos_sub_agent_jobs DROP COLUMN IF EXISTS tenant_id;
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose sqlite3
ALTER TABLE kairos_sub_agent_jobs DROP COLUMN tenant_id;
ALTER TABLE kairos_state_transitions DROP COLUMN tenant_id;
-- +goose StatementEnd
