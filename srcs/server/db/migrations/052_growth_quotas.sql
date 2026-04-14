CREATE TABLE IF NOT EXISTS growth_quotas (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    used INTEGER DEFAULT 0,
    max INTEGER DEFAULT 100,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(organization_id, resource_type)
);
