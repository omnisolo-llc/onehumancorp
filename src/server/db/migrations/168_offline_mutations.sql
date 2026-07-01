-- Create offline_mutations table for idempotent sync
CREATE TABLE IF NOT EXISTS offline_mutations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    idempotency_key VARCHAR(255) NOT NULL,
    entity_type VARCHAR(50) NOT NULL,
    entity_id UUID NOT NULL,
    action VARCHAR(50) NOT NULL,
    payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    processed_at TIMESTAMP WITH TIME ZONE,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    error_message TEXT,
    CONSTRAINT offline_mutations_idempotency_key_tenant_id_key UNIQUE (tenant_id, idempotency_key)
);

-- Enable RLS
ALTER TABLE offline_mutations ENABLE ROW LEVEL SECURITY;

-- Create policies
CREATE POLICY "offline_mutations_tenant_isolation" ON offline_mutations
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Create indexes
CREATE INDEX idx_offline_mutations_tenant_id ON offline_mutations(tenant_id);
CREATE INDEX idx_offline_mutations_status ON offline_mutations(status);
