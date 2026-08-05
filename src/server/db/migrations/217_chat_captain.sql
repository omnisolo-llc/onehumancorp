CREATE TABLE IF NOT EXISTS captain_assistants (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE captain_assistants ENABLE ROW LEVEL SECURITY;
CREATE POLICY captain_assistants_tenant_isolation_policy ON captain_assistants FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
CREATE UNIQUE INDEX IF NOT EXISTS idx_captain_assistants_tenant_name ON captain_assistants (tenant_id, name);

CREATE TABLE IF NOT EXISTS captain_documents (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    assistant_id UUID NOT NULL REFERENCES captain_assistants(id) ON DELETE CASCADE,
    name TEXT,
    external_link TEXT NOT NULL,
    content TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE captain_documents ENABLE ROW LEVEL SECURITY;
CREATE POLICY captain_documents_tenant_isolation_policy ON captain_documents FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
CREATE UNIQUE INDEX IF NOT EXISTS idx_captain_documents_assistant_external_link ON captain_documents (assistant_id, external_link);

CREATE TABLE IF NOT EXISTS captain_assistant_responses (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    assistant_id UUID NOT NULL REFERENCES captain_assistants(id) ON DELETE CASCADE,
    document_id UUID REFERENCES captain_documents(id) ON DELETE SET NULL,
    question TEXT NOT NULL,
    answer TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
ALTER TABLE captain_assistant_responses ENABLE ROW LEVEL SECURITY;
CREATE POLICY captain_assistant_responses_tenant_isolation_policy ON captain_assistant_responses FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
