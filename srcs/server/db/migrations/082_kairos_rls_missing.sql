-- +goose Up
-- +goose StatementBegin
-- +goose postgres
ALTER TABLE kairos_state_transitions ENABLE ROW LEVEL SECURITY;
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose postgres
ALTER TABLE kairos_sub_agent_jobs ENABLE ROW LEVEL SECURITY;
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose postgres
ALTER TABLE agent_missions ENABLE ROW LEVEL SECURITY;
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose postgres
DROP POLICY IF EXISTS tenant_isolation_kairos_state_transitions ON kairos_state_transitions;
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose postgres
CREATE POLICY tenant_isolation_kairos_state_transitions ON kairos_state_transitions
    USING (
        task_id IN (
            SELECT id FROM kairos_shared_tasks WHERE tenant_id = nullif(current_setting('app.current_tenant', true), '')::uuid
        )
    );
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose postgres
DROP POLICY IF EXISTS tenant_isolation_kairos_sub_agent_jobs ON kairos_sub_agent_jobs;
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose postgres
CREATE POLICY tenant_isolation_kairos_sub_agent_jobs ON kairos_sub_agent_jobs
    USING (
        parent_task_id IN (
            SELECT id FROM kairos_shared_tasks WHERE tenant_id = nullif(current_setting('app.current_tenant', true), '')::uuid
        )
    );
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose postgres
DROP POLICY IF EXISTS tenant_isolation_agent_missions ON agent_missions;
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose postgres
CREATE POLICY tenant_isolation_agent_missions ON agent_missions
    USING (organization_id = nullif(current_setting('app.current_tenant', true), ''));
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- +goose postgres
DROP POLICY IF EXISTS tenant_isolation_agent_missions ON agent_missions;
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose postgres
ALTER TABLE agent_missions DISABLE ROW LEVEL SECURITY;
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose postgres
DROP POLICY IF EXISTS tenant_isolation_kairos_sub_agent_jobs ON kairos_sub_agent_jobs;
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose postgres
ALTER TABLE kairos_sub_agent_jobs DISABLE ROW LEVEL SECURITY;
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose postgres
DROP POLICY IF EXISTS tenant_isolation_kairos_state_transitions ON kairos_state_transitions;
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose postgres
ALTER TABLE kairos_state_transitions DISABLE ROW LEVEL SECURITY;
-- +goose StatementEnd
