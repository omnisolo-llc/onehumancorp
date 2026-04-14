-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN crdt_vector JSONB;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE shared_tasks DROP COLUMN crdt_vector;
-- +goose StatementEnd
