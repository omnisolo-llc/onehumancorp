CREATE TABLE IF NOT EXISTS inventory_ledger (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    catalog_item_id TEXT NOT NULL,
    variant_id TEXT NOT NULL,
    change_amount INT NOT NULL,
    reason TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    UNIQUE(tenant_id, transaction_id)
);

CREATE TABLE IF NOT EXISTS agent_actions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    interaction_id TEXT,
    action_type TEXT NOT NULL,
    payload JSONB NOT NULL
);
