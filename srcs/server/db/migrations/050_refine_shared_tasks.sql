-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS shared_tasks (id VARCHAR PRIMARY KEY);
ALTER TABLE shared_tasks ADD COLUMN ultraplan_phase VARCHAR;
ALTER TABLE shared_tasks ADD COLUMN deliberation_log TEXT;
ALTER TABLE shared_tasks ADD COLUMN depth INTEGER DEFAULT 0;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- Down not used in tests
-- +goose StatementEnd
