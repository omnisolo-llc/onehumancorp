-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN priority TEXT NOT NULL DEFAULT 'P2';
ALTER TABLE shared_tasks ADD COLUMN locked_until TIMESTAMP;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE shared_tasks DROP COLUMN priority;
ALTER TABLE shared_tasks DROP COLUMN locked_until;
-- +goose StatementEnd
