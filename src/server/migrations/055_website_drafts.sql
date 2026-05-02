-- Create website_drafts table
CREATE TABLE IF NOT EXISTS website_drafts (
    organization_id UUID NOT NULL,
    draft_version INTEGER NOT NULL DEFAULT 1,
    live_version INTEGER,
    blocks JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (organization_id)
);

-- Enable RLS
ALTER TABLE website_drafts ENABLE ROW LEVEL SECURITY;

-- Create policy for website_drafts
CREATE POLICY tenant_isolation_website_drafts ON website_drafts
    USING (organization_id = current_setting('app.current_tenant_id', true)::uuid);
