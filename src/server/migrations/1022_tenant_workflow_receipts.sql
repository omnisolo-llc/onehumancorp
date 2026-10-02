-- Durable admitted text receipts. This does not authorize provider dispatch.
-- Raw tenant IDs are retained; no UUID remapping or system context is used.
CREATE UNIQUE INDEX IF NOT EXISTS tenant_receipt_user_identity ON users(id,tenant_id);
CREATE TABLE IF NOT EXISTS tenant_workflow_receipts (
 id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
 tenant_id TEXT NOT NULL CHECK(length(tenant_id) BETWEEN 1 AND 512 AND btrim(tenant_id)=tenant_id AND lower(tenant_id)<>'system'),
 actor_id TEXT NOT NULL CHECK(length(actor_id) BETWEEN 1 AND 512),
 request_id TEXT NOT NULL CHECK(length(request_id)=36),
 fingerprint TEXT NOT NULL CHECK(fingerprint ~ '^[a-f0-9]{64}$'),
 payload TEXT NOT NULL CHECK(jsonb_typeof(payload::jsonb)='object' AND octet_length(payload)<=200000),
 token_id TEXT NOT NULL CHECK(length(token_id) BETWEEN 1 AND 512),
 token_expires_at BIGINT NOT NULL CHECK(token_expires_at>created_at),
 session_id TEXT CHECK(session_id IS NULL OR length(session_id) BETWEEN 1 AND 512),
 phase TEXT NOT NULL CHECK(phase IN ('queued','dispatching','completed','cancelled','outcome_unknown')),
 generation BIGINT NOT NULL CHECK(generation>=0),
 lease TEXT CHECK(lease IS NULL OR length(lease)=36),
 lease_expires_at BIGINT CHECK(lease_expires_at IS NULL OR lease_expires_at>created_at),
 created_at BIGINT NOT NULL CHECK(created_at>0),
 updated_at BIGINT NOT NULL CHECK(updated_at>=created_at),
 output TEXT CHECK(output IS NULL OR octet_length(output)<=64000),
 error TEXT CHECK(error IS NULL OR error IN ('authority_lost','cancelled','execution_uncertain','lease_expired')),
 UNIQUE(tenant_id,actor_id,request_id),
 FOREIGN KEY(actor_id,tenant_id) REFERENCES users(id,tenant_id) ON DELETE CASCADE,
 CHECK((lease IS NULL)=(lease_expires_at IS NULL)),
 CHECK((phase='queued' AND generation=0 AND lease IS NULL)
    OR (phase='dispatching' AND generation=1 AND lease IS NOT NULL)
    OR (phase='completed' AND generation=2 AND lease IS NOT NULL)
    OR (phase='outcome_unknown' AND generation=2 AND lease IS NOT NULL)
    OR (phase='cancelled' AND ((generation=1 AND lease IS NULL) OR (generation=2 AND lease IS NOT NULL)))),
 CHECK((phase='completed' AND output IS NOT NULL AND length(btrim(output))>0 AND error IS NULL AND updated_at<lease_expires_at)
    OR (phase IN ('queued','dispatching') AND output IS NULL AND error IS NULL)
    OR (phase IN ('cancelled','outcome_unknown') AND output IS NULL AND error IS NOT NULL))
);
ALTER TABLE tenant_workflow_receipts ENABLE ROW LEVEL SECURITY;
ALTER TABLE tenant_workflow_receipts FORCE ROW LEVEL SECURITY;
CREATE POLICY tenant_workflow_receipt_scope ON tenant_workflow_receipts
 USING(tenant_id=current_setting('app.current_tenant',true))
 WITH CHECK(tenant_id=current_setting('app.current_tenant',true));
CREATE FUNCTION tenant_receipt_admission() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
 IF NEW.phase<>'queued' OR NEW.generation<>0 OR NEW.lease IS NOT NULL THEN
  RAISE EXCEPTION 'receipt_requires_queued_admission';
 END IF;
 RETURN NEW;
END $$;
CREATE TRIGGER tenant_receipt_admission BEFORE INSERT ON tenant_workflow_receipts
 FOR EACH ROW EXECUTE FUNCTION tenant_receipt_admission();
CREATE FUNCTION tenant_receipt_transition() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
 IF ROW(NEW.id,NEW.tenant_id,NEW.actor_id,NEW.request_id,NEW.fingerprint,NEW.payload,
        NEW.token_id,NEW.token_expires_at,NEW.session_id,NEW.created_at)
    IS DISTINCT FROM ROW(OLD.id,OLD.tenant_id,OLD.actor_id,OLD.request_id,OLD.fingerprint,OLD.payload,
        OLD.token_id,OLD.token_expires_at,OLD.session_id,OLD.created_at) THEN
  RAISE EXCEPTION 'receipt_admission_is_immutable';
 END IF;
 IF NOT ((OLD.phase='queued' AND NEW.phase IN ('dispatching','cancelled'))
      OR (OLD.phase='dispatching' AND NEW.phase IN ('completed','cancelled','outcome_unknown')))
    OR NEW.generation<>OLD.generation+1 OR NEW.updated_at<OLD.updated_at
    OR (OLD.lease IS NOT NULL AND ROW(NEW.lease,NEW.lease_expires_at) IS DISTINCT FROM ROW(OLD.lease,OLD.lease_expires_at)) THEN
  RAISE EXCEPTION 'receipt_transition_is_invalid';
 END IF;
 RETURN NEW;
END $$;
CREATE TRIGGER tenant_receipt_transition BEFORE UPDATE ON tenant_workflow_receipts
 FOR EACH ROW EXECUTE FUNCTION tenant_receipt_transition();
CREATE FUNCTION tenant_receipt_delete() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
 IF EXISTS(SELECT 1 FROM users WHERE id=OLD.actor_id AND tenant_id=OLD.tenant_id) THEN
  RAISE EXCEPTION 'receipt_cannot_be_discarded';
 END IF;
 RETURN OLD;
END $$;
CREATE TRIGGER tenant_receipt_delete BEFORE DELETE ON tenant_workflow_receipts
 FOR EACH ROW EXECUTE FUNCTION tenant_receipt_delete();
-- Runtime-created temporary tables must not shadow the canonical parent table.
DO $$ DECLARE function_name TEXT; BEGIN
 FOREACH function_name IN ARRAY ARRAY['tenant_receipt_admission','tenant_receipt_transition','tenant_receipt_delete'] LOOP
  EXECUTE format('ALTER FUNCTION %I.%I() SET search_path TO pg_catalog,%I,pg_temp',current_schema(),function_name,current_schema());
 END LOOP;
END $$;
