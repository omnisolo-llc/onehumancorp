-- +goose Up
ALTER TABLE checkout_sessions ADD COLUMN IF NOT EXISTS target_currency TEXT;
ALTER TABLE checkout_sessions ADD COLUMN IF NOT EXISTS base_currency TEXT DEFAULT 'USD';
