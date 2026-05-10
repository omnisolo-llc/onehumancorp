-- +goose Up
-- +goose StatementBegin
-- +goose postgres
ALTER TABLE kairos_state_transitions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_kairos_state_transitions ON kairos_state_transitions;
CREATE POLICY tenant_isolation_kairos_state_transitions ON kairos_state_transitions
    USING (task_id IN (
        SELECT id FROM kairos_shared_tasks
        WHERE tenant_id = nullif(current_setting('app.current_tenant', true), '')::uuid
    ));

ALTER TABLE kairos_sub_agent_jobs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_kairos_sub_agent_jobs ON kairos_sub_agent_jobs;
CREATE POLICY tenant_isolation_kairos_sub_agent_jobs ON kairos_sub_agent_jobs
    USING (parent_task_id IN (
        SELECT id FROM kairos_shared_tasks
        WHERE tenant_id = nullif(current_setting('app.current_tenant', true), '')::uuid
    ));

ALTER TABLE sub_agent_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_sub_agent_queue ON sub_agent_queue;
CREATE POLICY tenant_isolation_sub_agent_queue ON sub_agent_queue
    USING (organization_id = current_setting('app.current_tenant', true));

ALTER TABLE agent_missions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_missions ON agent_missions;
CREATE POLICY tenant_isolation_agent_missions ON agent_missions
    USING (organization_id = current_setting('app.current_tenant', true));

ALTER TABLE telemetry_buffer ADD COLUMN IF NOT EXISTS organization_id VARCHAR NOT NULL DEFAULT 'system';
ALTER TABLE telemetry_buffer ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_telemetry_buffer ON telemetry_buffer;
CREATE POLICY tenant_isolation_telemetry_buffer ON telemetry_buffer
    USING (organization_id = current_setting('app.current_tenant', true));

ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks
    USING (mission_id IN (
        SELECT id FROM agent_missions
        WHERE organization_id = current_setting('app.current_tenant', true)
    ));

ALTER TABLE state_machine_transitions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_state_machine_transitions ON state_machine_transitions;
CREATE POLICY tenant_isolation_state_machine_transitions ON state_machine_transitions
    USING (entity_id IN (
        SELECT id FROM swarm_tasks
        WHERE mission_id IN (
            SELECT id FROM agent_missions
            WHERE organization_id = current_setting('app.current_tenant', true)
        )
    ));
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose sqlite3
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- +goose postgres
DROP POLICY IF EXISTS tenant_isolation_state_machine_transitions ON state_machine_transitions;
ALTER TABLE state_machine_transitions DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
ALTER TABLE swarm_tasks DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_telemetry_buffer ON telemetry_buffer;
ALTER TABLE telemetry_buffer DISABLE ROW LEVEL SECURITY;
ALTER TABLE telemetry_buffer DROP COLUMN IF EXISTS organization_id;

DROP POLICY IF EXISTS tenant_isolation_agent_missions ON agent_missions;
ALTER TABLE agent_missions DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_sub_agent_queue ON sub_agent_queue;
ALTER TABLE sub_agent_queue DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_kairos_sub_agent_jobs ON kairos_sub_agent_jobs;
ALTER TABLE kairos_sub_agent_jobs DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_kairos_state_transitions ON kairos_state_transitions;
ALTER TABLE kairos_state_transitions DISABLE ROW LEVEL SECURITY;
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose sqlite3
-- +goose StatementEnd
