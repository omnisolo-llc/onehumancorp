-- +goose Up
-- +goose StatementBegin
-- +goose postgres
ALTER TABLE agent_missions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_missions ON agent_missions;
CREATE POLICY tenant_isolation_agent_missions ON agent_missions
    USING (organization_id = nullif(current_setting('app.current_tenant', true), ''));

ALTER TABLE kairos_state_transitions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_kairos_state_transitions ON kairos_state_transitions;
CREATE POLICY tenant_isolation_kairos_state_transitions ON kairos_state_transitions
    USING (
        EXISTS (
            SELECT 1 FROM kairos_shared_tasks kst
            WHERE kst.id = kairos_state_transitions.task_id
            AND kst.tenant_id = nullif(current_setting('app.current_tenant', true), '')::uuid
        )
    );

ALTER TABLE kairos_sub_agent_jobs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_kairos_sub_agent_jobs ON kairos_sub_agent_jobs;
CREATE POLICY tenant_isolation_kairos_sub_agent_jobs ON kairos_sub_agent_jobs
    USING (
        EXISTS (
            SELECT 1 FROM kairos_shared_tasks kst
            WHERE kst.id = kairos_sub_agent_jobs.parent_task_id
            AND kst.tenant_id = nullif(current_setting('app.current_tenant', true), '')::uuid
        )
    );
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose sqlite3
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- +goose postgres
DROP POLICY IF EXISTS tenant_isolation_agent_missions ON agent_missions;
ALTER TABLE agent_missions DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_kairos_state_transitions ON kairos_state_transitions;
ALTER TABLE kairos_state_transitions DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_kairos_sub_agent_jobs ON kairos_sub_agent_jobs;
ALTER TABLE kairos_sub_agent_jobs DISABLE ROW LEVEL SECURITY;
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose sqlite3
-- +goose StatementEnd
