-- +goose Up
-- Add base_currency to tenants
ALTER TABLE tenants ADD COLUMN IF NOT EXISTS base_currency TEXT DEFAULT 'USD';
