-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS wizard_drafts (
    user_id TEXT PRIMARY KEY,
    state JSONB NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE wizard_drafts ENABLE ROW LEVEL SECURITY;
-- +goose StatementEnd
