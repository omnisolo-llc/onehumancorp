-- +goose Up
ALTER TABLE IF EXISTS distributed_locks ENABLE ROW LEVEL SECURITY;

-- +goose Down
-- Cannot easily downgrade RLS, typically left alone
