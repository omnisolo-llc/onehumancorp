-- Add verifiable receipts to the existing queue; legacy rows remain unverified.
ALTER TABLE sync_events ADD COLUMN IF NOT EXISTS request_identity JSONB;
ALTER TABLE sync_events ADD COLUMN IF NOT EXISTS receipt_status TEXT;
ALTER TABLE sync_events ADD COLUMN IF NOT EXISTS receipt_route TEXT;
ALTER TABLE sync_events ADD COLUMN IF NOT EXISTS client_event_id TEXT;
ALTER TABLE sync_events ADD COLUMN IF NOT EXISTS entity_type TEXT;
ALTER TABLE sync_events ADD COLUMN IF NOT EXISTS entity_id TEXT;
ALTER TABLE sync_events ADD COLUMN IF NOT EXISTS base_version BIGINT;
ALTER TABLE sync_events ADD COLUMN IF NOT EXISTS result_version BIGINT;
CREATE INDEX IF NOT EXISTS sync_events_entity_receipts
    ON sync_events (tenant_id, entity_type, entity_id, result_version)
    WHERE receipt_status = 'acknowledged';
CREATE TABLE IF NOT EXISTS operation_intents (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    action_type TEXT NOT NULL,
    payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retry_count INT NOT NULL DEFAULT 0
);
ALTER TABLE operation_intents ENABLE ROW LEVEL SECURITY;
DO $$ BEGIN
IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE schemaname=current_schema() AND tablename='operation_intents' AND policyname='tenant_isolation_operation_intents') THEN
CREATE POLICY tenant_isolation_operation_intents ON operation_intents USING (tenant_id=current_setting('app.current_tenant',true)) WITH CHECK (tenant_id=current_setting('app.current_tenant',true));
END IF; END $$;
ALTER TABLE operation_intents ADD COLUMN IF NOT EXISTS request_identity JSONB;

-- Existing online writers do not all assign updated_at. Maintain the observed
-- concurrency token at the database boundary, including non-sync updates.
CREATE OR REPLACE FUNCTION touch_sync_entity_updated_at() RETURNS trigger
LANGUAGE plpgsql AS $$ BEGIN NEW.updated_at = GREATEST(clock_timestamp(), OLD.updated_at + INTERVAL '1 microsecond'); RETURN NEW; END $$;
DROP TRIGGER IF EXISTS sync_entity_updated_at ON products;
CREATE TRIGGER sync_entity_updated_at BEFORE UPDATE ON products
FOR EACH ROW EXECUTE FUNCTION touch_sync_entity_updated_at();
DROP TRIGGER IF EXISTS sync_entity_updated_at ON orders;
CREATE TRIGGER sync_entity_updated_at BEFORE UPDATE ON orders
FOR EACH ROW EXECUTE FUNCTION touch_sync_entity_updated_at();
DROP TRIGGER IF EXISTS sync_entity_updated_at ON appointments;
CREATE TRIGGER sync_entity_updated_at BEFORE UPDATE ON appointments
FOR EACH ROW EXECUTE FUNCTION touch_sync_entity_updated_at();
