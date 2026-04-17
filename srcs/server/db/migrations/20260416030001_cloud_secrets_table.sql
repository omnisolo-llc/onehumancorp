-- Create cloud_secrets table for multi-tenant cloud mode credential sync
CREATE TABLE IF NOT EXISTS cloud_secrets (
    org_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT,
    PRIMARY KEY (org_id, key)
);
