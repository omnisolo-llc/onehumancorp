-- Collaboration Tables
CREATE TABLE IF NOT EXISTS bundle_products (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    price_cents BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS bundle_items (
    bundle_id TEXT NOT NULL REFERENCES bundle_products(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    split_cents BIGINT NOT NULL,
    PRIMARY KEY (bundle_id, product_id, tenant_id)
);

CREATE TABLE IF NOT EXISTS unified_carts (
    id TEXT PRIMARY KEY,
    customer_id TEXT NOT NULL,
    total_cents BIGINT NOT NULL,
    status TEXT NOT NULL, -- PENDING, CHECKOUT_INITIATED, RESERVED, PAID, COMPLETED
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS cart_items (
    id TEXT PRIMARY KEY,
    cart_id TEXT NOT NULL REFERENCES unified_carts(id) ON DELETE CASCADE,
    bundle_id TEXT NOT NULL REFERENCES bundle_products(id),
    quantity INT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Note: In a real environment, we would also ensure cross-tenant RLS
-- but for a shared collaboration engine table, it may exist in a "public" or
-- trusted system schema that the `CollaborationService` accesses.
