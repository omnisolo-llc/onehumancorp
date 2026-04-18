-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS autodream_memories (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    embedding TEXT,
    source_mission_id TEXT,
    organization_id TEXT,
    agent_id TEXT,
    source_type TEXT,
    processed_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS autodream_memories;
-- +goose StatementEnd
