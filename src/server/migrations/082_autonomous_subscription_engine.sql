CREATE TABLE IF NOT EXISTS subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR NOT NULL,
    customer_id VARCHAR NOT NULL,
    plan_id VARCHAR NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'active',
    crdt_vector JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS subscription_usage_ledger (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR NOT NULL,
    subscription_id UUID NOT NULL REFERENCES subscriptions(id),
    usage_delta INT NOT NULL,
    idempotency_key VARCHAR UNIQUE NOT NULL,
    synced_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE subscriptions ENABLE ROW LEVEL SECURITY;
ALTER TABLE subscription_usage_ledger ENABLE ROW LEVEL SECURITY;
