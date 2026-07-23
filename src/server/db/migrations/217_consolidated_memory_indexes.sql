-- +goose Up
-- Indexes to optimize prune_stale queries in multi-tenant system
CREATE INDEX IF NOT EXISTS consolidated_memory_prune_source_idx
    ON consolidated_memory (tenant_id, source_type, owner_override, last_referenced_at);

CREATE INDEX IF NOT EXISTS consolidated_memory_prune_reliability_idx
    ON consolidated_memory (tenant_id, reliability_score, owner_override, last_referenced_at);

-- +goose Down
DROP INDEX IF EXISTS consolidated_memory_prune_source_idx;
DROP INDEX IF EXISTS consolidated_memory_prune_reliability_idx;
