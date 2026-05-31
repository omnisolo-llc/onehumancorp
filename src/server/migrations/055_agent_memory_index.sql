-- +goose NO TRANSACTION
-- +goose Up
-- Migration 055: Add composite index to agent_memories for low latency history retrieval
CREATE INDEX CONCURRENTLY idx_agent_memories_history ON agent_memories(tenant_id, customer_id, created_at);

-- +goose Down
DROP INDEX CONCURRENTLY IF EXISTS idx_agent_memories_history;
