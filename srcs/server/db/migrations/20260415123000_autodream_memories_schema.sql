-- +goose Up
-- +goose StatementBegin
ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS metadata JSONB;
CREATE INDEX IF NOT EXISTS autodream_memories_embedding_idx ON autodream_memories USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP INDEX IF EXISTS autodream_memories_embedding_idx;
ALTER TABLE autodream_memories DROP COLUMN IF EXISTS metadata;
ALTER TABLE autodream_memories DROP COLUMN IF EXISTS organization_id;
-- +goose StatementEnd
