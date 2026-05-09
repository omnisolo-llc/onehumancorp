-- +goose Up
ALTER TABLE consolidated_memory ADD COLUMN last_accessed_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP;
CREATE INDEX idx_consolidated_memory_accessed ON consolidated_memory(last_accessed_at);

-- +goose Down
DROP INDEX IF EXISTS idx_consolidated_memory_accessed;
ALTER TABLE consolidated_memory DROP COLUMN last_accessed_at;
