-- Exact local field-write receipts; no provider dispatch or inferred completion.
CREATE TABLE field_mutation_receipts (
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    operation TEXT NOT NULL CHECK (operation IN ('appointment','routing_job','route')),
    idempotency_key TEXT NOT NULL CHECK (octet_length(idempotency_key) BETWEEN 1 AND 128),
    actor_id TEXT NOT NULL,
    request_sha256 TEXT NOT NULL CHECK (request_sha256 ~ '^[0-9a-f]{64}$'),
    response JSONB NOT NULL CHECK (jsonb_typeof(response) = 'object'),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, operation, idempotency_key)
);
ALTER TABLE field_mutation_receipts ENABLE ROW LEVEL SECURITY;
ALTER TABLE field_mutation_receipts FORCE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_field_mutation_receipts ON field_mutation_receipts
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
