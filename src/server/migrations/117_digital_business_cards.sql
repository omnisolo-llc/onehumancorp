CREATE TABLE IF NOT EXISTS growth_digital_cards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    title TEXT NOT NULL,
    company TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT,
    bio TEXT,
    website TEXT,
    theme TEXT DEFAULT 'light',
    vcard_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Row Level Security
ALTER TABLE growth_digital_cards ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_policy ON growth_digital_cards
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE POLICY public_read_policy ON growth_digital_cards
    FOR SELECT
    TO public
    USING (true); -- Public cards should be viewable by anyone with the link
