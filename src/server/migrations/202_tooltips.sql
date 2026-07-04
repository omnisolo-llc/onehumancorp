CREATE TABLE IF NOT EXISTS tooltips (
    id TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    text TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id, tenant_id)
);

ALTER TABLE tooltips ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Tenant isolation for tooltips" ON tooltips
    USING (tenant_id = current_setting('app.current_tenant', true));

-- Seed some default tooltips for the default tenant that E2E tests expect
INSERT INTO tooltips (id, tenant_id, text) VALUES
    ('api-docs-tooltip', 'default', 'Direct API access is only for custom integrations.'),
    ('settings-delivery-tooltip', 'default', 'Turn this on to offer local delivery to your customers.'),
    ('help-btn-tooltip', 'default', 'Need help? Click here to access our Help Center, Ask AI, Video Tutorials, and Release Notes.'),
    ('help-search-tooltip', 'default', 'Search for help articles and videos...'),
    ('ask-ai-tooltip', 'default', 'Open AI Help Chat to get answers instantly.')
ON CONFLICT (id, tenant_id) DO UPDATE SET text = EXCLUDED.text;

-- Also seed for the 'ohc' tenant often used in testing
INSERT INTO tooltips (id, tenant_id, text) VALUES
    ('api-docs-tooltip', 'ohc', 'Direct API access is only for custom integrations.'),
    ('settings-delivery-tooltip', 'ohc', 'Turn this on to offer local delivery to your customers.'),
    ('help-btn-tooltip', 'ohc', 'Need help? Click here to access our Help Center, Ask AI, Video Tutorials, and Release Notes.'),
    ('help-search-tooltip', 'ohc', 'Search for help articles and videos...'),
    ('ask-ai-tooltip', 'ohc', 'Open AI Help Chat to get answers instantly.')
ON CONFLICT (id, tenant_id) DO UPDATE SET text = EXCLUDED.text;

-- And for e2e
INSERT INTO tooltips (id, tenant_id, text) VALUES
    ('api-docs-tooltip', 'e2e', 'Direct API access is only for custom integrations.'),
    ('settings-delivery-tooltip', 'e2e', 'Turn this on to offer local delivery to your customers.'),
    ('help-btn-tooltip', 'e2e', 'Need help? Click here to access our Help Center, Ask AI, Video Tutorials, and Release Notes.'),
    ('help-search-tooltip', 'e2e', 'Search for help articles and videos...'),
    ('ask-ai-tooltip', 'e2e', 'Open AI Help Chat to get answers instantly.')
ON CONFLICT (id, tenant_id) DO UPDATE SET text = EXCLUDED.text;

-- And for e2e_smoke_help-features
INSERT INTO tooltips (id, tenant_id, text) VALUES
    ('api-docs-tooltip', 'e2e_smoke_help-features', 'Direct API access is only for custom integrations.'),
    ('settings-delivery-tooltip', 'e2e_smoke_help-features', 'Turn this on to offer local delivery to your customers.'),
    ('help-btn-tooltip', 'e2e_smoke_help-features', 'Need help? Click here to access our Help Center, Ask AI, Video Tutorials, and Release Notes.'),
    ('help-search-tooltip', 'e2e_smoke_help-features', 'Search for help articles and videos...'),
    ('ask-ai-tooltip', 'e2e_smoke_help-features', 'Open AI Help Chat to get answers instantly.')
ON CONFLICT (id, tenant_id) DO UPDATE SET text = EXCLUDED.text;
