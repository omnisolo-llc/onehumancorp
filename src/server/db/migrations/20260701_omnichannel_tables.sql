CREATE TABLE IF NOT EXISTS customer_profile (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE customer_profile ENABLE ROW LEVEL SECURITY;
CREATE POLICY customer_profile_tenant_isolation_policy ON customer_profile FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS work_item (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    customer_id UUID NOT NULL REFERENCES customer_profile(id),
    source TEXT NOT NULL,
    payload JSONB,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE work_item ENABLE ROW LEVEL SECURITY;
CREATE POLICY work_item_tenant_isolation_policy ON work_item FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS agent_draft (
    id UUID PRIMARY KEY,
    work_item_id UUID NOT NULL REFERENCES work_item(id),
    response TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE agent_draft ENABLE ROW LEVEL SECURITY;
CREATE POLICY agent_draft_tenant_isolation_policy ON agent_draft FOR ALL USING (
    EXISTS (
        SELECT 1 FROM work_item WHERE work_item.id = agent_draft.work_item_id AND work_item.tenant_id = current_setting('app.current_tenant_id', true)::uuid
    )
);
