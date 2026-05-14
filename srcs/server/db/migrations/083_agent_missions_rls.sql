-- +goose Up
-- +goose StatementBegin
-- +goose postgres
ALTER TABLE agent_missions ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_agent_missions ON agent_missions;
CREATE POLICY tenant_isolation_agent_missions ON agent_missions
    USING (organization_id = current_setting('app.current_tenant', true));
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose sqlite3
-- +goose StatementEnd


-- +goose Down
-- +goose StatementBegin
-- +goose postgres
DROP POLICY IF EXISTS tenant_isolation_agent_missions ON agent_missions;
ALTER TABLE agent_missions DISABLE ROW LEVEL SECURITY;
-- +goose StatementEnd

-- +goose StatementBegin
-- +goose sqlite3
-- +goose StatementEnd
