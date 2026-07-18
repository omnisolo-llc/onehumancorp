-- +goose Up
CREATE TABLE IF NOT EXISTS customer_memory_context (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    context_graph JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE customer_memory_context ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_customer_memory_context ON customer_memory_context
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE UNIQUE INDEX idx_customer_memory_context_tenant_customer ON customer_memory_context (tenant_id, customer_id);

-- +goose Down
DROP TABLE IF EXISTS customer_memory_context CASCADE;
