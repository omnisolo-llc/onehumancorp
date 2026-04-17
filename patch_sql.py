import re

# Update SQL for postgres and sqlite compatibility

import os

with open('srcs/server/db/migrations/20260416060000_autodream_vector_pipeline.sql', 'w') as f:
    f.write("""-- +goose Up
-- +goose StatementBegin
CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS consolidated_memory (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    agent_id TEXT,
    content TEXT NOT NULL,
    embedding VECTOR(1536),
    source_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_consolidated_memory_embedding ON consolidated_memory USING hnsw (embedding vector_cosine_ops);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS consolidated_memory;
-- +goose StatementEnd
""")

with open('srcs/server/db/migrations/20260416060000_autodream_vector_pipeline_sqlite.sql', 'w') as f:
    f.write("""-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS consolidated_memory (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    agent_id TEXT,
    content TEXT NOT NULL,
    embedding TEXT,
    source_type TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS consolidated_memory;
-- +goose StatementEnd
""")
os.rename('srcs/server/db/migrations/20260416060000_autodream_vector_pipeline.sql', 'srcs/server/db/migrations/20260416060000_autodream_vector_pipeline_pg.sql')
