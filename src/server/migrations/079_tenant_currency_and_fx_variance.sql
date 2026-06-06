-- Add base currency and FX variance bucket for Offline-First Multi-Currency Engine
ALTER TABLE tenants
ADD COLUMN IF NOT EXISTS tenant_base_currency TEXT NOT NULL DEFAULT 'USD',
ADD COLUMN IF NOT EXISTS fx_variance_bucket BIGINT NOT NULL DEFAULT 0;
