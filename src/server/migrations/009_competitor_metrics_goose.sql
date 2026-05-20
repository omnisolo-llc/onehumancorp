-- +goose Up
-- +goose StatementBegin
-- Creating a dummy goose migration for competitor_metrics as instructed. Table already created in 002_missing_tables.sql
SELECT 1;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
SELECT 1;
-- +goose StatementEnd
