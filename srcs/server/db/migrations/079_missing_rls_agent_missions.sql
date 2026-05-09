-- +goose Up
ALTER TABLE agent_missions ENABLE ROW LEVEL SECURITY;

-- +goose Down
ALTER TABLE agent_missions DISABLE ROW LEVEL SECURITY;
