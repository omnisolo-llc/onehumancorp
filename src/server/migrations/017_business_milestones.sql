CREATE TABLE IF NOT EXISTS business_milestones (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    milestone_type TEXT NOT NULL,
    reached_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    shared_at TIMESTAMP,
    metadata JSONB DEFAULT '{}',
    UNIQUE(tenant_id, milestone_type)
);

CREATE INDEX IF NOT EXISTS idx_business_milestones_tenant_id ON business_milestones(tenant_id);
