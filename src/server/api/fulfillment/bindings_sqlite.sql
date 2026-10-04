CREATE TABLE IF NOT EXISTS delivery_provider_bindings (
    delivery_task_id TEXT PRIMARY KEY REFERENCES delivery_tasks(id),
    organization_id TEXT NOT NULL,
    provider TEXT NOT NULL CHECK (provider IN ('shippo', 'doordash')),
    account_namespace TEXT NOT NULL CHECK (length(account_namespace) BETWEEN 1 AND 128),
    provider_object_id TEXT NOT NULL CHECK (length(provider_object_id) BETWEEN 1 AND 256),
    label_url TEXT,
    tracking_number TEXT,
    carrier TEXT,
    is_test BOOLEAN NOT NULL,
    last_event_at_ms INTEGER,
    last_event_digest TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (organization_id, provider, account_namespace, is_test, provider_object_id)
);
CREATE INDEX IF NOT EXISTS idx_delivery_binding_tracking
ON delivery_provider_bindings (organization_id, provider, account_namespace, is_test, carrier, tracking_number);
