-- +goose Up
-- Migration 216: Add InventoryReservation and extend SyncEvent for Omnichannel POS Sync

CREATE TABLE IF NOT EXISTS inventory_reservations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    session_id TEXT NOT NULL,
    quantity INT NOT NULL DEFAULT 1,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_inventory_reservations_tenant_product ON inventory_reservations(tenant_id, product_id);
CREATE INDEX IF NOT EXISTS idx_inventory_reservations_expires_at ON inventory_reservations(expires_at);

DO $$
BEGIN
    IF to_regclass('inventory_reservations') IS NOT NULL THEN
        ALTER TABLE inventory_reservations ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'inventory_reservations'
                AND policyname = 'tenant_isolation_inventory_reservations'
        ) THEN
            CREATE POLICY tenant_isolation_inventory_reservations ON inventory_reservations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;

-- Add fallback sync_status to pos_offline_transactions if not exists
ALTER TABLE pos_offline_transactions ADD COLUMN IF NOT EXISTS sync_status TEXT DEFAULT 'pending';

-- +goose Down
DO $$
BEGIN
    IF to_regclass('inventory_reservations') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_inventory_reservations ON inventory_reservations;
        ALTER TABLE inventory_reservations DISABLE ROW LEVEL SECURITY;
    END IF;
END
$$;

DROP TABLE IF EXISTS inventory_reservations CASCADE;
