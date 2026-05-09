-- +goose Up
-- +goose StatementBegin
-- +goose postgres
ALTER TABLE kairos_shared_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_kairos_shared_tasks ON kairos_shared_tasks;
CREATE POLICY tenant_isolation_kairos_shared_tasks ON kairos_shared_tasks
    USING (tenant_id = nullif(current_setting('app.current_tenant', true), '')::uuid);

ALTER TABLE autodream_vector_memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_autodream_vector_memories ON autodream_vector_memories;
CREATE POLICY tenant_isolation_autodream_vector_memories ON autodream_vector_memories
    USING (tenant_id = nullif(current_setting('app.current_tenant', true), '')::uuid);
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose sqlite3
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- +goose postgres
DROP POLICY IF EXISTS tenant_isolation_kairos_shared_tasks ON kairos_shared_tasks;
ALTER TABLE kairos_shared_tasks DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_autodream_vector_memories ON autodream_vector_memories;
ALTER TABLE autodream_vector_memories DISABLE ROW LEVEL SECURITY;
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose sqlite3
-- +goose StatementEnd
