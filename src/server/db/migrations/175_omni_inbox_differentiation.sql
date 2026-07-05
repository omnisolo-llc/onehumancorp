-- +goose Up

-- Extend customer profile concept for omnichannel if missing
CREATE TABLE IF NOT EXISTS customer_profile (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    name TEXT,
    email TEXT,
    phone TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('customer_profile') IS NOT NULL THEN
        ALTER TABLE customer_profile ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies WHERE tablename = 'customer_profile' AND policyname = 'tenant_isolation_customer_profile'
        ) THEN
            CREATE POLICY tenant_isolation_customer_profile ON customer_profile USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;


CREATE TABLE IF NOT EXISTS work_item (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id UUID NOT NULL REFERENCES customer_profile(id) ON DELETE CASCADE,
    source TEXT NOT NULL,
    payload JSONB,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('work_item') IS NOT NULL THEN
        ALTER TABLE work_item ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies WHERE tablename = 'work_item' AND policyname = 'tenant_isolation_work_item'
        ) THEN
            CREATE POLICY tenant_isolation_work_item ON work_item USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;

CREATE TABLE IF NOT EXISTS agent_draft (
    id UUID PRIMARY KEY,
    work_item_id UUID NOT NULL REFERENCES work_item(id) ON DELETE CASCADE,
    response TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- We need a trigger or complex policy for agent_draft if it doesn't have tenant_id directly.
-- Let's add tenant_id to make RLS simple and fast.
ALTER TABLE agent_draft ADD COLUMN IF NOT EXISTS tenant_id TEXT;

-- For existing rows if any
UPDATE agent_draft ad SET tenant_id = wi.tenant_id FROM work_item wi WHERE ad.work_item_id = wi.id AND ad.tenant_id IS NULL;

-- Now require it
-- ALTER TABLE agent_draft ALTER COLUMN tenant_id SET NOT NULL; -- skipping strict NOT NULL for existing schema compatibility

DO $$
BEGIN
    IF to_regclass('agent_draft') IS NOT NULL THEN
        ALTER TABLE agent_draft ENABLE ROW LEVEL SECURITY;
        IF NOT EXISTS (
            SELECT 1 FROM pg_policies WHERE tablename = 'agent_draft' AND policyname = 'tenant_isolation_agent_draft'
        ) THEN
            CREATE POLICY tenant_isolation_agent_draft ON agent_draft USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
        END IF;
    END IF;
END
$$;


-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_agent_draft ON agent_draft;
    DROP POLICY IF EXISTS tenant_isolation_work_item ON work_item;
    DROP POLICY IF EXISTS tenant_isolation_customer_profile ON customer_profile;
END
$$;

ALTER TABLE agent_draft DROP COLUMN IF EXISTS tenant_id;

DROP TABLE IF EXISTS agent_draft CASCADE;
DROP TABLE IF EXISTS work_item CASCADE;
DROP TABLE IF EXISTS customer_profile CASCADE;
