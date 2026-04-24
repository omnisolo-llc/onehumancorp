-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS wizard_drafts (
    user_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL DEFAULT 'system',
    state TEXT NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS wizard_drafts;
-- +goose StatementEnd
