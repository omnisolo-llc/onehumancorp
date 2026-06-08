-- +goose Up
-- Migration 108: Inventory levels and Agent Action Requests

CREATE TABLE IF NOT EXISTS inventory_levels (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    location TEXT NOT NULL, -- 'online' or 'in-store'
    quantity INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('inventory_levels') IS NOT NULL THEN
        ALTER TABLE inventory_levels ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'inventory_levels'
                AND policyname = 'tenant_isolation_inventory_levels'
        ) THEN
            CREATE POLICY tenant_isolation_inventory_levels ON inventory_levels USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS agent_action_requests (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    action_type TEXT NOT NULL, -- e.g., 'Reorder', 'PriceAdjust'
    status TEXT NOT NULL DEFAULT 'Pending', -- 'Pending', 'Approved', 'Rejected'
    confidence_score DECIMAL DEFAULT 0,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    payload JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('agent_action_requests') IS NOT NULL THEN
        ALTER TABLE agent_action_requests ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1
            FROM pg_policies
            WHERE schemaname = current_schema()
                AND tablename = 'agent_action_requests'
                AND policyname = 'tenant_isolation_agent_action_requests'
        ) THEN
            CREATE POLICY tenant_isolation_agent_action_requests ON agent_action_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('agent_action_requests') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_agent_action_requests ON agent_action_requests;
        ALTER TABLE agent_action_requests DISABLE ROW LEVEL SECURITY;
    END IF;

    IF to_regclass('inventory_levels') IS NOT NULL THEN
        DROP POLICY IF EXISTS tenant_isolation_inventory_levels ON inventory_levels;
        ALTER TABLE inventory_levels DISABLE ROW LEVEL SECURITY;
    END IF;
END
$$;

DROP TABLE IF EXISTS agent_action_requests CASCADE;
DROP TABLE IF EXISTS inventory_levels CASCADE;
