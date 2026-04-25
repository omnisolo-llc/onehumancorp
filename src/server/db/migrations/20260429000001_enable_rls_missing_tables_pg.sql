-- +goose Up
-- +goose StatementBegin
ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE swarm_tasks DISABLE ROW LEVEL SECURITY;
-- +goose StatementEnd
