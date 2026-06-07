-- Migration 087: Drop primary key from _sqlx_migrations to fix duplicate key error when running migrations on top of old seed db
ALTER TABLE _sqlx_migrations DROP CONSTRAINT IF EXISTS _sqlx_migrations_pkey;
