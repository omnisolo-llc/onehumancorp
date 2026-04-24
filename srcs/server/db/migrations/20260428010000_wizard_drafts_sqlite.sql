-- +goose Up
CREATE TABLE wizard_drafts (
    tenant_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    state_json TEXT NOT NULL DEFAULT '{}',
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, user_id)
);

-- +goose Down
DROP TABLE wizard_drafts;
