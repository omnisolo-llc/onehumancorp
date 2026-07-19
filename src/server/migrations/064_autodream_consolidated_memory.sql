-- +goose Up
-- SQL in this section is executed when the migration is applied.
CREATE INDEX IF NOT EXISTS consolidated_memory_tenant_idx ON consolidated_memory(tenant_id);
