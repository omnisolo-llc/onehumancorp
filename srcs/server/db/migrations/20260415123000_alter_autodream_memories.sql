-- +goose Up
-- +goose StatementBegin
ALTER TABLE autodream_memories ADD COLUMN organization_id VARCHAR;
UPDATE autodream_memories SET organization_id = 'default' WHERE organization_id IS NULL;
ALTER TABLE autodream_memories ALTER COLUMN organization_id SET NOT NULL;

ALTER TABLE autodream_memories ADD COLUMN metadata JSONB;
CREATE INDEX IF NOT EXISTS autodream_memories_embedding_idx ON autodream_memories USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP INDEX IF EXISTS autodream_memories_embedding_idx;
ALTER TABLE autodream_memories DROP COLUMN metadata;
ALTER TABLE autodream_memories DROP COLUMN organization_id;
-- +goose StatementEnd
