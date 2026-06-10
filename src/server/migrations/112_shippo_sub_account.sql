-- Add Shippo sub_account_id to organizations
ALTER TABLE organizations ADD COLUMN IF NOT EXISTS shippo_account_id text;
