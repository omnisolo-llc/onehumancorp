-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS shared_tasks_v2 (
    id VARCHAR PRIMARY KEY,
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    dependencies TEXT NOT NULL DEFAULT '[]'
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS shared_tasks_v2;
-- +goose StatementEnd
