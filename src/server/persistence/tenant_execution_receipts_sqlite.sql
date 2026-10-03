-- Explicit portable initialization on the configured connection. This schema
-- stores admitted text work; it grants no provider, tool or workspace access.
CREATE UNIQUE INDEX IF NOT EXISTS tenant_receipt_user_identity ON users(id,tenant_id);
CREATE TABLE IF NOT EXISTS tenant_workflow_receipts (
 id TEXT PRIMARY KEY NOT NULL CHECK(length(id)=36),
 tenant_id TEXT NOT NULL CHECK(length(tenant_id) BETWEEN 1 AND 512 AND trim(tenant_id)=tenant_id AND lower(tenant_id)<>'system'),
 actor_id TEXT NOT NULL CHECK(length(actor_id) BETWEEN 1 AND 512),
 request_id TEXT NOT NULL CHECK(length(request_id)=36),
 fingerprint TEXT NOT NULL CHECK(length(fingerprint)=64 AND fingerprint NOT GLOB '*[^a-f0-9]*'),
 payload TEXT NOT NULL CHECK(json_valid(payload) AND json_type(payload)='object' AND length(CAST(payload AS BLOB))<=200000),
 token_id TEXT NOT NULL CHECK(length(token_id) BETWEEN 1 AND 512),
 token_expires_at INTEGER NOT NULL CHECK(typeof(token_expires_at)='integer' AND token_expires_at>created_at),
 session_id TEXT CHECK(session_id IS NULL OR length(session_id) BETWEEN 1 AND 512),
 phase TEXT NOT NULL CHECK(phase IN ('queued','dispatching','completed','cancelled','outcome_unknown')),
 generation INTEGER NOT NULL CHECK(typeof(generation)='integer' AND generation>=0),
 lease TEXT CHECK(lease IS NULL OR length(lease)=36),
 lease_expires_at INTEGER CHECK(lease_expires_at IS NULL OR (typeof(lease_expires_at)='integer' AND lease_expires_at>created_at)),
 created_at INTEGER NOT NULL CHECK(typeof(created_at)='integer' AND created_at>0),
 updated_at INTEGER NOT NULL CHECK(typeof(updated_at)='integer' AND updated_at>=created_at),
 output TEXT CHECK(output IS NULL OR length(CAST(output AS BLOB))<=64000),
 error TEXT CHECK(error IS NULL OR error IN ('authority_lost','cancelled','execution_uncertain','lease_expired')),
 UNIQUE(tenant_id,actor_id,request_id),
 FOREIGN KEY(actor_id,tenant_id) REFERENCES users(id,tenant_id) ON DELETE CASCADE,
 CHECK((lease IS NULL)=(lease_expires_at IS NULL)),
 CHECK((phase='queued' AND generation=0 AND lease IS NULL)
    OR (phase='dispatching' AND generation=1 AND lease IS NOT NULL)
    OR (phase='completed' AND generation=2 AND lease IS NOT NULL)
    OR (phase='outcome_unknown' AND generation=2 AND lease IS NOT NULL)
    OR (phase='cancelled' AND ((generation=1 AND lease IS NULL) OR (generation=2 AND lease IS NOT NULL)))),
 CHECK((phase='completed' AND output IS NOT NULL AND length(trim(output))>0 AND error IS NULL AND updated_at<lease_expires_at)
    OR (phase IN ('queued','dispatching') AND output IS NULL AND error IS NULL)
    OR (phase IN ('cancelled','outcome_unknown') AND output IS NULL AND error IS NOT NULL))
);
CREATE TRIGGER IF NOT EXISTS tenant_receipt_admission BEFORE INSERT ON tenant_workflow_receipts
WHEN NEW.phase<>'queued' OR NEW.generation<>0 OR NEW.lease IS NOT NULL
 OR EXISTS(SELECT 1 FROM tenant_workflow_receipts WHERE id=NEW.id
   OR (tenant_id=NEW.tenant_id AND actor_id=NEW.actor_id AND request_id=NEW.request_id))
BEGIN SELECT RAISE(ABORT,'receipt_requires_queued_admission'); END;
CREATE TRIGGER IF NOT EXISTS tenant_receipt_immutable_identity BEFORE UPDATE ON tenant_workflow_receipts
WHEN NEW.id IS NOT OLD.id OR NEW.tenant_id IS NOT OLD.tenant_id OR NEW.actor_id IS NOT OLD.actor_id
 OR NEW.request_id IS NOT OLD.request_id OR NEW.fingerprint IS NOT OLD.fingerprint OR NEW.payload IS NOT OLD.payload
 OR NEW.token_id IS NOT OLD.token_id OR NEW.token_expires_at IS NOT OLD.token_expires_at
 OR NEW.session_id IS NOT OLD.session_id OR NEW.created_at IS NOT OLD.created_at
BEGIN SELECT RAISE(ABORT,'receipt_admission_is_immutable'); END;
CREATE TRIGGER IF NOT EXISTS tenant_receipt_transition BEFORE UPDATE ON tenant_workflow_receipts
WHEN NOT ((OLD.phase='queued' AND NEW.phase IN ('dispatching','cancelled'))
 OR (OLD.phase='dispatching' AND NEW.phase IN ('completed','cancelled','outcome_unknown')))
 OR NEW.generation<>OLD.generation+1 OR NEW.updated_at<OLD.updated_at
 OR (OLD.lease IS NOT NULL AND (NEW.lease IS NOT OLD.lease OR NEW.lease_expires_at IS NOT OLD.lease_expires_at))
BEGIN SELECT RAISE(ABORT,'receipt_transition_is_invalid'); END;
CREATE TRIGGER IF NOT EXISTS tenant_receipt_delete BEFORE DELETE ON tenant_workflow_receipts
WHEN EXISTS(SELECT 1 FROM users WHERE id=OLD.actor_id AND tenant_id=OLD.tenant_id)
BEGIN SELECT RAISE(ABORT,'receipt_cannot_be_discarded'); END;
