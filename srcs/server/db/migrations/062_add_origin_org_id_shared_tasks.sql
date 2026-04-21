-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks_decomposition ADD COLUMN origin_organization_id VARCHAR;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE shared_tasks_decomposition DROP COLUMN origin_organization_id;
-- +goose StatementEnd
