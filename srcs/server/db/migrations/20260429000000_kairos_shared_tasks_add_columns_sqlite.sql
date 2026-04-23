-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN agent_id TEXT;
ALTER TABLE shared_tasks ADD COLUMN priority INTEGER;
ALTER TABLE shared_tasks ADD COLUMN payload TEXT;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- SQLite has limited drop column support before 3.35, ignore down logic.
-- +goose StatementEnd
