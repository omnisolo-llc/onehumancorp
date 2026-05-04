CREATE TABLE IF NOT EXISTS storefronts (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL,
    user_id             TEXT NOT NULL,
    template            TEXT NOT NULL,
    primary_color       TEXT NOT NULL,
    product_name        TEXT NOT NULL,
    product_price       TEXT NOT NULL,
    product_description TEXT NOT NULL,
    domain_choice       TEXT NOT NULL,
    blocks              TEXT NOT NULL DEFAULT '[]',
    status              TEXT NOT NULL DEFAULT 'published',
    seo_metadata        TEXT NOT NULL DEFAULT '',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, organization_id)
);

ALTER TABLE storefronts ENABLE ROW LEVEL SECURITY;
