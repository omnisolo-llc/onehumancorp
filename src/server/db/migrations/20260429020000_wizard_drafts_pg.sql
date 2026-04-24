-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS wizard_drafts (
    user_id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL DEFAULT 'system',
    state JSONB NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose StatementBegin
ALTER TABLE wizard_drafts ENABLE ROW LEVEL SECURITY;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS wizard_drafts;
-- +goose StatementEnd
