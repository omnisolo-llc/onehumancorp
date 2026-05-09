-- +goose Up
-- +goose StatementBegin
-- +goose postgres
ALTER TABLE kairos_shared_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE kairos_state_transitions ENABLE ROW LEVEL SECURITY;
ALTER TABLE kairos_sub_agent_jobs ENABLE ROW LEVEL SECURITY;
ALTER TABLE autodream_vector_memories ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_kairos_shared_tasks ON kairos_shared_tasks;
CREATE POLICY tenant_isolation_kairos_shared_tasks ON kairos_shared_tasks
    USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

DROP POLICY IF EXISTS tenant_isolation_kairos_state_transitions ON kairos_state_transitions;
CREATE POLICY tenant_isolation_kairos_state_transitions ON kairos_state_transitions
    USING (task_id IN (SELECT id FROM kairos_shared_tasks WHERE tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system'));

DROP POLICY IF EXISTS tenant_isolation_kairos_sub_agent_jobs ON kairos_sub_agent_jobs;
CREATE POLICY tenant_isolation_kairos_sub_agent_jobs ON kairos_sub_agent_jobs
    USING (parent_task_id IN (SELECT id FROM kairos_shared_tasks WHERE tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system'));

DROP POLICY IF EXISTS tenant_isolation_autodream_vector_memories ON autodream_vector_memories;
CREATE POLICY tenant_isolation_autodream_vector_memories ON autodream_vector_memories
    USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- +goose postgres
DROP POLICY IF EXISTS tenant_isolation_autodream_vector_memories ON autodream_vector_memories;
DROP POLICY IF EXISTS tenant_isolation_kairos_sub_agent_jobs ON kairos_sub_agent_jobs;
DROP POLICY IF EXISTS tenant_isolation_kairos_state_transitions ON kairos_state_transitions;
DROP POLICY IF EXISTS tenant_isolation_kairos_shared_tasks ON kairos_shared_tasks;

ALTER TABLE autodream_vector_memories DISABLE ROW LEVEL SECURITY;
ALTER TABLE kairos_sub_agent_jobs DISABLE ROW LEVEL SECURITY;
ALTER TABLE kairos_state_transitions DISABLE ROW LEVEL SECURITY;
ALTER TABLE kairos_shared_tasks DISABLE ROW LEVEL SECURITY;
-- +goose StatementEnd
