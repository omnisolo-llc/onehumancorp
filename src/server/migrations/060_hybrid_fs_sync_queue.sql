CREATE TABLE IF NOT EXISTS hybrid_fs_sync_queue (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    local_path TEXT NOT NULL,
    cloud_path TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'FILE_SYNC_PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
