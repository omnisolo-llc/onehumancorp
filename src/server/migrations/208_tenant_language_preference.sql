-- Migration: Add language_preference to tenants

ALTER TABLE IF EXISTS tenants ADD COLUMN IF NOT EXISTS language_preference TEXT DEFAULT 'en';
