-- Keep cloud/Postgres subscription workers on the same schema contract as the
-- standalone store, and ensure the least-privilege system role can access
-- tables created after the role's original migration.

ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS synced_to_cloud BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS cloud_mission_id TEXT;
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS sync_error TEXT;
ALTER TABLE agent_missions ADD COLUMN IF NOT EXISTS last_synced_at TIMESTAMPTZ;
ALTER TABLE agent_missions ALTER COLUMN synced_to_cloud SET DEFAULT FALSE;
UPDATE agent_missions SET synced_to_cloud = FALSE WHERE synced_to_cloud IS NULL;
ALTER TABLE agent_missions ALTER COLUMN synced_to_cloud SET NOT NULL;

CREATE TABLE IF NOT EXISTS subscription_plans (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    product_id TEXT,
    name TEXT NOT NULL DEFAULT '',
    description TEXT,
    price_cents BIGINT NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    frequency TEXT NOT NULL DEFAULT 'month',
    interval TEXT,
    discount_percentage DOUBLE PRECISION,
    cutoff_day INTEGER,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE subscription_plans ADD COLUMN IF NOT EXISTS product_id TEXT;
ALTER TABLE subscription_plans ADD COLUMN IF NOT EXISTS description TEXT;
ALTER TABLE subscription_plans ADD COLUMN IF NOT EXISTS interval TEXT;
ALTER TABLE subscription_plans ADD COLUMN IF NOT EXISTS discount_percentage DOUBLE PRECISION;
ALTER TABLE subscription_plans ADD COLUMN IF NOT EXISTS status TEXT NOT NULL DEFAULT 'active';

CREATE TABLE IF NOT EXISTS subscribers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    subscription_plan_id TEXT NOT NULL REFERENCES subscription_plans(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    stripe_subscription_id TEXT,
    health_score INTEGER NOT NULL DEFAULT 100,
    last_engagement_at TIMESTAMPTZ,
    predicted_restock_date BIGINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE subscribers ADD COLUMN IF NOT EXISTS health_score INTEGER NOT NULL DEFAULT 100;
ALTER TABLE subscribers ADD COLUMN IF NOT EXISTS last_engagement_at TIMESTAMPTZ;
ALTER TABLE subscribers ADD COLUMN IF NOT EXISTS predicted_restock_date BIGINT;

CREATE TABLE IF NOT EXISTS subscriptions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    plan_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    health_score DOUBLE PRECISION,
    current_period_start TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    current_period_end TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    cancel_at_period_end BOOLEAN NOT NULL DEFAULT FALSE,
    canceled_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS fulfillment_schedules (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    subscription_plan_id TEXT NOT NULL,
    fulfillment_date DATE NOT NULL,
    subscriber_count INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE subscriptions ADD COLUMN IF NOT EXISTS health_score DOUBLE PRECISION;
ALTER TABLE orders ADD COLUMN IF NOT EXISTS is_consumable BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE orders ADD COLUMN IF NOT EXISTS estimated_duration_days INTEGER;

ALTER TABLE subscription_plans ENABLE ROW LEVEL SECURITY;
ALTER TABLE subscribers ENABLE ROW LEVEL SECURITY;
ALTER TABLE subscriptions ENABLE ROW LEVEL SECURITY;
ALTER TABLE fulfillment_schedules ENABLE ROW LEVEL SECURITY;
ALTER TABLE subscription_plans FORCE ROW LEVEL SECURITY;
ALTER TABLE subscribers FORCE ROW LEVEL SECURITY;
ALTER TABLE subscriptions FORCE ROW LEVEL SECURITY;
ALTER TABLE fulfillment_schedules FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_subscription_plans ON subscription_plans;
CREATE POLICY tenant_isolation_subscription_plans ON subscription_plans
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
DROP POLICY IF EXISTS tenant_isolation_subscribers ON subscribers;
CREATE POLICY tenant_isolation_subscribers ON subscribers
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
DROP POLICY IF EXISTS tenant_isolation_subscriptions ON subscriptions;
CREATE POLICY tenant_isolation_subscriptions ON subscriptions
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
DROP POLICY IF EXISTS tenant_isolation_fulfillment_schedules ON fulfillment_schedules;
CREATE POLICY tenant_isolation_fulfillment_schedules ON fulfillment_schedules
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE UNIQUE INDEX IF NOT EXISTS uq_subscription_plans_id_tenant
    ON subscription_plans (id, tenant_id);
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint
        WHERE conname = 'subscribers_plan_tenant_fk'
          AND conrelid = 'subscribers'::regclass
    ) THEN
        ALTER TABLE subscribers
            ADD CONSTRAINT subscribers_plan_tenant_fk
            FOREIGN KEY (subscription_plan_id, tenant_id)
            REFERENCES subscription_plans (id, tenant_id)
            ON DELETE CASCADE NOT VALID;
    END IF;
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint
        WHERE conname = 'subscriptions_plan_tenant_fk'
          AND conrelid = 'subscriptions'::regclass
    ) THEN
        ALTER TABLE subscriptions
            ADD CONSTRAINT subscriptions_plan_tenant_fk
            FOREIGN KEY (plan_id, tenant_id)
            REFERENCES subscription_plans (id, tenant_id)
            ON DELETE CASCADE NOT VALID;
    END IF;
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint
        WHERE conname = 'fulfillment_schedules_plan_tenant_fk'
          AND conrelid = 'fulfillment_schedules'::regclass
    ) THEN
        ALTER TABLE fulfillment_schedules
            ADD CONSTRAINT fulfillment_schedules_plan_tenant_fk
            FOREIGN KEY (subscription_plan_id, tenant_id)
            REFERENCES subscription_plans (id, tenant_id)
            ON DELETE CASCADE NOT VALID;
    END IF;
END
$$;

CREATE INDEX IF NOT EXISTS idx_agent_missions_pending_sync
    ON agent_missions (tenant_id, synced_to_cloud, created_at)
    WHERE synced_to_cloud = FALSE;
CREATE INDEX IF NOT EXISTS idx_agent_missions_active_cloud_sync
    ON agent_missions (tenant_id, synced_to_cloud, status)
    WHERE cloud_mission_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_subscription_plans_tenant
    ON subscription_plans (tenant_id, status);
CREATE INDEX IF NOT EXISTS idx_subscribers_tenant_plan_status
    ON subscribers (tenant_id, subscription_plan_id, status);
CREATE INDEX IF NOT EXISTS idx_subscribers_stripe_subscription
    ON subscribers (stripe_subscription_id)
    WHERE stripe_subscription_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_subscriptions_tenant_status_period
    ON subscriptions (tenant_id, status, current_period_end);
CREATE INDEX IF NOT EXISTS idx_fulfillment_schedules_tenant_date
    ON fulfillment_schedules (tenant_id, fulfillment_date, status);
CREATE INDEX IF NOT EXISTS idx_orders_consumable_restock
    ON orders (is_consumable, estimated_duration_days, created_at)
    WHERE is_consumable = TRUE AND estimated_duration_days IS NOT NULL;

ALTER ROLE ohc_bypassrls NOLOGIN BYPASSRLS;
GRANT USAGE ON SCHEMA public TO ohc_bypassrls;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO ohc_bypassrls;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO ohc_bypassrls;
ALTER DEFAULT PRIVILEGES IN SCHEMA public
    GRANT ALL PRIVILEGES ON TABLES TO ohc_bypassrls;
ALTER DEFAULT PRIVILEGES IN SCHEMA public
    GRANT ALL PRIVILEGES ON SEQUENCES TO ohc_bypassrls;
GRANT ohc_bypassrls TO CURRENT_USER;
