-- Create local_secrets table for Standalone Mode offline credential caching
CREATE TABLE IF NOT EXISTS local_secrets (
    id TEXT PRIMARY KEY,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    synced_to_cloud BOOLEAN DEFAULT true
);
