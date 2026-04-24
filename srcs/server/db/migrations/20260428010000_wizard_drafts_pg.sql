-- +goose Up
CREATE TABLE wizard_drafts (
    tenant_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    state_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY (tenant_id, user_id)
);
ALTER TABLE wizard_drafts ENABLE ROW LEVEL SECURITY;

-- +goose Down
DROP TABLE wizard_drafts;
