-- Preserve the original request and signature. Null identity marks legacy
-- transactions whose side effects cannot safely be inferred or replayed.
ALTER TABLE pos_offline_transactions ADD COLUMN IF NOT EXISTS request_identity JSONB;
ALTER TABLE pos_offline_transactions ADD COLUMN IF NOT EXISTS request_status TEXT;
ALTER TABLE pos_offline_transactions ADD COLUMN IF NOT EXISTS request_reconciliation JSONB;
ALTER TABLE pos_offline_transactions ADD COLUMN IF NOT EXISTS device_signature TEXT;
ALTER TABLE pos_offline_transactions ADD COLUMN IF NOT EXISTS terminal_id TEXT;
