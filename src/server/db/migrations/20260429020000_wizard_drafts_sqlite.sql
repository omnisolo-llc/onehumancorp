-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS wizard_drafts (
    user_id TEXT PRIMARY KEY,
    state JSONB NOT NULL,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd
