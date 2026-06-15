-- Migration 131: Add _sync_status to pos_offline_transactions
ALTER TABLE pos_offline_transactions ADD COLUMN IF NOT EXISTS _sync_status VARCHAR(50) DEFAULT 'pending';
