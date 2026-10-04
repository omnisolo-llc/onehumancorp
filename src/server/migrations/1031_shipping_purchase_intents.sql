-- One durable purchase lifecycle per canonical tenant/order. Unknown dispatches
-- must be reconciled; a new caller key or account configuration cannot reset it.
CREATE TABLE IF NOT EXISTS shipping_purchase_intents (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    order_id TEXT NOT NULL,
    rate_id TEXT NOT NULL,
    account_namespace TEXT NOT NULL,
    is_test BOOLEAN NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('dispatched', 'recorded')),
    transaction_id TEXT,
    receipt_json TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (organization_id, order_id),
    UNIQUE (organization_id, account_namespace, is_test, transaction_id),
    CHECK ((status = 'recorded' AND transaction_id IS NOT NULL AND receipt_json IS NOT NULL)
        OR (status = 'dispatched' AND receipt_json IS NULL))
);
ALTER TABLE shipping_purchase_intents ENABLE ROW LEVEL SECURITY;
ALTER TABLE shipping_purchase_intents FORCE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_shipping_purchase_intents ON shipping_purchase_intents
USING (organization_id = current_setting('app.current_tenant', true))
WITH CHECK (organization_id = current_setting('app.current_tenant', true));

CREATE OR REPLACE FUNCTION shipping_purchase_identity_immutable() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    IF ROW(OLD.id, OLD.organization_id, OLD.actor_id, OLD.order_id, OLD.rate_id,
           OLD.account_namespace, OLD.is_test, OLD.created_at)
       IS DISTINCT FROM
       ROW(NEW.id, NEW.organization_id, NEW.actor_id, NEW.order_id, NEW.rate_id,
           NEW.account_namespace, NEW.is_test, NEW.created_at)
       OR (OLD.transaction_id IS NOT NULL AND OLD.transaction_id IS DISTINCT FROM NEW.transaction_id)
       OR (OLD.status = 'recorded' AND ROW(OLD.status, OLD.receipt_json)
           IS DISTINCT FROM ROW(NEW.status, NEW.receipt_json)) THEN
        RAISE EXCEPTION 'shipping purchase identity and recorded receipt are immutable';
    END IF;
    RETURN NEW;
END;
$$;
CREATE TRIGGER shipping_purchase_identity_immutable
BEFORE UPDATE ON shipping_purchase_intents
FOR EACH ROW EXECUTE FUNCTION shipping_purchase_identity_immutable();
