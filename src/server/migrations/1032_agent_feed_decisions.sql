-- A decision and its queue admission commit together. attempted_at is a durable
-- one-way dispatch fence, not a provider success/delivery acknowledgement.
CREATE TABLE agent_feed_decisions (
    tenant_id TEXT NOT NULL,
    action_id TEXT NOT NULL,
    origin TEXT NOT NULL,
    decision_state TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    token_id TEXT NOT NULL,
    job_id TEXT UNIQUE,
    dispatch_payload JSONB,
    dispatch_status TEXT NOT NULL CHECK (dispatch_status IN
      ('NOT_REQUESTED','PENDING','ATTEMPTING','DISPATCH_RETURNED','RECONCILIATION_REQUIRED','CANCELLED')),
    attempted_at TIMESTAMPTZ,
    dispatch_returned_at TIMESTAMPTZ,
    detail TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, action_id),
    CHECK (attempted_at IS NULL OR dispatch_status IN ('ATTEMPTING','DISPATCH_RETURNED','RECONCILIATION_REQUIRED'))
);
CREATE INDEX agent_feed_decisions_pending ON agent_feed_decisions(dispatch_status,updated_at);
ALTER TABLE agent_feed_decisions ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_feed_decisions FORCE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_feed_decisions ON agent_feed_decisions
USING (tenant_id = current_setting('app.current_tenant',true))
WITH CHECK (tenant_id = current_setting('app.current_tenant',true));

-- Keep the older feed mirror's edited payload/context consistent with the
-- canonical decision. Nested synchronization remains guarded against recursion.
CREATE OR REPLACE FUNCTION sync_items_to_agent_feed()
RETURNS TRIGGER AS $$
BEGIN
    IF pg_trigger_depth() > 1 THEN RETURN NEW; END IF;
    UPDATE agent_feed
    SET state = NEW.lifecycle_state,
        payload = NEW.proposed_action,
        description = COALESCE(NEW.context_payload->>'description',description),
        title = COALESCE(NEW.context_payload->>'title',title),
        priority = COALESCE(NEW.context_payload->>'priority',priority),
        updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id AND tenant_id = NEW.tenant_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
DROP TRIGGER IF EXISTS trg_items_sync_agent_feed ON agent_feed_items;
CREATE TRIGGER trg_items_sync_agent_feed
AFTER UPDATE OF lifecycle_state,context_payload,proposed_action ON agent_feed_items
FOR EACH ROW EXECUTE FUNCTION sync_items_to_agent_feed();

-- Ordinary updates cannot erase the dispatch fence or retarget an admitted job.
CREATE FUNCTION protect_agent_feed_dispatch_fence() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        IF OLD.attempted_at IS NOT NULL OR OLD.dispatch_status='RECONCILIATION_REQUIRED' THEN
            RAISE EXCEPTION 'Agent feed dispatch attempt evidence cannot be deleted' USING ERRCODE='23514';
        END IF;
        RETURN OLD;
    END IF;
    IF NEW.tenant_id IS DISTINCT FROM OLD.tenant_id
       OR NEW.action_id IS DISTINCT FROM OLD.action_id
       OR NEW.origin IS DISTINCT FROM OLD.origin
       OR (OLD.job_id IS NOT NULL AND NEW.job_id IS DISTINCT FROM OLD.job_id)
       OR (OLD.attempted_at IS NOT NULL AND (
           NEW.attempted_at IS DISTINCT FROM OLD.attempted_at
           OR NEW.actor_id IS DISTINCT FROM OLD.actor_id
           OR NEW.token_id IS DISTINCT FROM OLD.token_id
           OR NEW.dispatch_payload IS DISTINCT FROM OLD.dispatch_payload))
       OR (OLD.dispatch_returned_at IS NOT NULL
           AND NEW.dispatch_returned_at IS DISTINCT FROM OLD.dispatch_returned_at)
    THEN
        RAISE EXCEPTION 'Agent feed dispatch identity and attempt evidence are immutable' USING ERRCODE='23514';
    END IF;
    RETURN NEW;
END $$;
CREATE TRIGGER protect_agent_feed_dispatch_fence BEFORE UPDATE OR DELETE ON agent_feed_decisions
FOR EACH ROW EXECUTE FUNCTION protect_agent_feed_dispatch_fence();
