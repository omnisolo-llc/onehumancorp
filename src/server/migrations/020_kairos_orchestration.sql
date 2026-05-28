-- KAIROS orchestration migrations
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS payload JSONB;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS result_payload JSONB;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT;
