-- +goose Up
-- +goose StatementBegin
ALTER TABLE agent_mesh_messages ENABLE ROW LEVEL SECURITY;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE agent_mesh_messages DISABLE ROW LEVEL SECURITY;
-- +goose StatementEnd
