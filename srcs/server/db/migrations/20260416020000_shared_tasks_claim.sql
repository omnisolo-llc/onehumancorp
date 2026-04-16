-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN claimed_by VARCHAR DEFAULT NULL;
ALTER TABLE shared_tasks ADD COLUMN claim_status VARCHAR DEFAULT NULL;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE shared_tasks DROP COLUMN claimed_by;
ALTER TABLE shared_tasks DROP COLUMN claim_status;
-- +goose StatementEnd
