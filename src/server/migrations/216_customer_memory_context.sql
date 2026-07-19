-- +goose Up
CREATE TABLE IF NOT EXISTS customer_memory_context (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    context_graph JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT customer_memory_context_tenant_customer_key UNIQUE (tenant_id, customer_id)
);

ALTER TABLE customer_memory_context ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_customer_memory_context ON customer_memory_context
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

<<<<<<< HEAD
-- +goose Down
DROP TABLE IF EXISTS customer_memory_context CASCADE;
=======
>>>>>>> 5b473f7d0 (feat: harden platform and real-data e2e)
