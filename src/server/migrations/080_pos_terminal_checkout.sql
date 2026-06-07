-- Migration 080: Add product_id to pos_offline_transactions

ALTER TABLE pos_offline_transactions ADD COLUMN IF NOT EXISTS product_id TEXT REFERENCES products(id) ON DELETE SET NULL;
