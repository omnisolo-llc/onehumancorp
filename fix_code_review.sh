#!/bin/bash
# 1. Drop redundant migration
git rm -f srcs/server/db/migrations/20260415123000_autodream_memories_schema.sql
sed -i 's/"migrations\/20260415123000_autodream_memories_schema.sql",//g' srcs/server/db/BUILD.bazel

# 2. Modify existing migration to add missing columns if needed
# We see 20260414130500_create_autodream_memories.sql was already created by another agent,
# and it lacks organization_id, metadata, and the index. We should create a new migration to ADD these.
cat << 'MIG' > srcs/server/db/migrations/20260415123000_alter_autodream_memories.sql
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
MIG

sed -i 's/"migrations\/20260414130500_create_autodream_memories.sql",/"migrations\/20260414130500_create_autodream_memories.sql",\n        "migrations\/20260415123000_alter_autodream_memories.sql",/g' srcs/server/db/BUILD.bazel

git add srcs/server/db/migrations/20260415123000_alter_autodream_memories.sql
git add srcs/server/db/BUILD.bazel
