-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS knowledge_embeddings (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    embedding TEXT,
    source_id TEXT,
    source_type TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE swarm_ultra_plans ADD COLUMN auto_dreamed BOOLEAN DEFAULT FALSE;
ALTER TABLE shared_tasks_decomposition ADD COLUMN auto_dreamed BOOLEAN DEFAULT FALSE;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE shared_tasks_decomposition DROP COLUMN auto_dreamed;
ALTER TABLE swarm_ultra_plans DROP COLUMN auto_dreamed;
DROP TABLE IF EXISTS knowledge_embeddings;
-- +goose StatementEnd
