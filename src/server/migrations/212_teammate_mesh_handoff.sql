-- +goose Up
-- Migration 212: Add Teammate Mesh Handoff fields to unified_threads

ALTER TABLE unified_threads ADD COLUMN IF NOT EXISTS lock_owner_id TEXT;
ALTER TABLE unified_threads ADD COLUMN IF NOT EXISTS lock_owner_type TEXT;

-- +goose Down
ALTER TABLE unified_threads DROP COLUMN IF EXISTS lock_owner_id;
ALTER TABLE unified_threads DROP COLUMN IF EXISTS lock_owner_type;
