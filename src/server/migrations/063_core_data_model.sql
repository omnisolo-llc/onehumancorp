-- 063_core_data_model.sql
-- Core data model architecture with multi-tenancy support for unified products and services.

CREATE TABLE IF NOT EXISTS catalog_items (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    item_type TEXT NOT NULL, -- Physical, Service, Digital, Subscription
    name TEXT NOT NULL,
    description TEXT,
    base_price_cents BIGINT NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_catalog_items_tenant ON catalog_items(tenant_id);

CREATE TABLE IF NOT EXISTS item_variants (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    catalog_item_id TEXT NOT NULL,
    name TEXT NOT NULL, -- e.g. "Size S, Red"
    price_cents BIGINT NOT NULL DEFAULT 0,
    attributes TEXT, -- JSON structure stored as text
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (catalog_item_id) REFERENCES catalog_items(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_item_variants_tenant ON item_variants(tenant_id);

CREATE TABLE IF NOT EXISTS inventory_ledger (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    item_variant_id TEXT NOT NULL,
    quantity_change BIGINT NOT NULL,
    reason TEXT NOT NULL, -- 'purchase', 'restock', 'adjustment'
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (item_variant_id) REFERENCES item_variants(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_inventory_ledger_tenant ON inventory_ledger(tenant_id);

CREATE TABLE IF NOT EXISTS order_lines (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    order_id TEXT NOT NULL, -- References orders.id
    item_variant_id TEXT NOT NULL,
    quantity BIGINT NOT NULL DEFAULT 1,
    unit_price_cents BIGINT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE,
    FOREIGN KEY (item_variant_id) REFERENCES item_variants(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_order_lines_tenant ON order_lines(tenant_id);

CREATE TABLE IF NOT EXISTS payments (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    order_id TEXT NOT NULL,
    amount_cents BIGINT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    status TEXT NOT NULL, -- 'pending', 'succeeded', 'failed', 'refunded'
    provider TEXT NOT NULL, -- 'stripe'
    provider_transaction_id TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_payments_tenant ON payments(tenant_id);

CREATE TABLE IF NOT EXISTS fulfillments (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    order_id TEXT NOT NULL,
    status TEXT NOT NULL, -- 'pending', 'shipped', 'delivered', 'completed'
    tracking_number TEXT,
    carrier TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_fulfillments_tenant ON fulfillments(tenant_id);

CREATE TABLE IF NOT EXISTS interactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    channel TEXT NOT NULL, -- 'instagram', 'email', 'website', 'whatsapp'
    content TEXT NOT NULL,
    direction TEXT NOT NULL, -- 'inbound', 'outbound'
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (customer_id) REFERENCES customers(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_interactions_tenant ON interactions(tenant_id);

CREATE TABLE IF NOT EXISTS agent_actions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    interaction_id TEXT,
    agent_id TEXT NOT NULL,
    action_type TEXT NOT NULL,
    status TEXT NOT NULL, -- 'draft', 'executed', 'failed'
    details TEXT, -- JSON structure
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (interaction_id) REFERENCES interactions(id) ON DELETE SET NULL,
    FOREIGN KEY (agent_id) REFERENCES agents(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_agent_actions_tenant ON agent_actions(tenant_id);
