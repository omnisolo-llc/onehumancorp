CREATE TABLE social_post_proposals (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    content TEXT NOT NULL,
    image_url TEXT,
    seo_alt_text TEXT,
    seo_meta_description TEXT,
    status TEXT NOT NULL, -- DRAFT, APPROVED, PUBLISHED, DISCARDED
    created_at_unix BIGINT NOT NULL,
    updated_at_unix BIGINT NOT NULL
);

ALTER TABLE social_post_proposals ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_social_post_proposals
ON social_post_proposals
FOR ALL
USING (tenant_id = current_setting('app.current_tenant_id'));
