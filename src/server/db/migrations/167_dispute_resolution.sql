CREATE TABLE IF NOT EXISTS dispute_evidence_packages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    dispute_id VARCHAR(255) NOT NULL,
    charge_id VARCHAR(255) NOT NULL,
    customer_id UUID REFERENCES customers(id) ON DELETE SET NULL,
    reason VARCHAR(255) NOT NULL,
    amount BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL,
    evidence_payload JSONB NOT NULL DEFAULT '{}',
    status VARCHAR(50) NOT NULL DEFAULT 'draft',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE dispute_evidence_packages ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_dispute_evidence_packages ON dispute_evidence_packages
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
