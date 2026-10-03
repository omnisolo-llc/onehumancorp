CREATE TABLE IF NOT EXISTS shipping_purchase_intents (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    order_id TEXT NOT NULL,
    rate_id TEXT NOT NULL,
    account_namespace TEXT NOT NULL,
    is_test INTEGER NOT NULL CHECK (is_test IN (0, 1)),
    status TEXT NOT NULL CHECK (status IN ('dispatched', 'recorded')),
    transaction_id TEXT,
    receipt_json TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (organization_id, order_id),
    UNIQUE (organization_id, account_namespace, is_test, transaction_id),
    CHECK ((status = 'recorded' AND transaction_id IS NOT NULL AND receipt_json IS NOT NULL)
        OR (status = 'dispatched' AND receipt_json IS NULL))
);
CREATE TRIGGER IF NOT EXISTS shipping_purchase_identity_immutable
BEFORE UPDATE ON shipping_purchase_intents
WHEN OLD.id IS NOT NEW.id OR OLD.organization_id IS NOT NEW.organization_id
    OR OLD.actor_id IS NOT NEW.actor_id OR OLD.order_id IS NOT NEW.order_id
    OR OLD.rate_id IS NOT NEW.rate_id OR OLD.account_namespace IS NOT NEW.account_namespace
    OR OLD.is_test IS NOT NEW.is_test OR OLD.created_at IS NOT NEW.created_at
    OR (OLD.transaction_id IS NOT NULL AND OLD.transaction_id IS NOT NEW.transaction_id)
    OR (OLD.status = 'recorded' AND (OLD.status IS NOT NEW.status OR OLD.receipt_json IS NOT NEW.receipt_json))
BEGIN
    SELECT RAISE(ABORT, 'shipping purchase identity and recorded receipt are immutable');
END;
