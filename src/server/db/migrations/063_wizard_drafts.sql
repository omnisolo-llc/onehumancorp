-- +goose Up
CREATE TABLE wizard_drafts (
    user_id TEXT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    draft_state TEXT NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementBegin
-- +goose StatementEnd
-- +goose Down
DROP TABLE IF EXISTS wizard_drafts;
