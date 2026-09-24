CREATE TABLE IF NOT EXISTS agent_feed (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    source TEXT NOT NULL,
    priority TEXT,
    title TEXT,
    description TEXT,
    payload JSONB,
    state TEXT NOT NULL DEFAULT 'PENDING_APPROVAL',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_agent_feed_tenant ON agent_feed(tenant_id);

ALTER TABLE agent_feed ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_agent_feed ON agent_feed;
CREATE POLICY tenant_isolation_agent_feed ON agent_feed
    FOR ALL
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE OR REPLACE FUNCTION sync_agent_feed_to_items()
RETURNS TRIGGER AS $$
BEGIN
    IF pg_trigger_depth() > 1 THEN
        RETURN NEW;
    END IF;
    INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
    VALUES (
        NEW.id,
        NEW.tenant_id,
        NEW.source,
        jsonb_build_object('description', NEW.description, 'title', NEW.title, 'priority', NEW.priority),
        CASE
            WHEN NEW.payload IS NULL THEN '{}'::jsonb
            WHEN jsonb_typeof(NEW.payload) = 'object' THEN NEW.payload
            ELSE jsonb_build_object('raw', NEW.payload)
        END,
        NEW.state,
        COALESCE(NEW.created_at, CURRENT_TIMESTAMP),
        COALESCE(NEW.updated_at, CURRENT_TIMESTAMP)
    )
    ON CONFLICT (id) DO UPDATE SET
        lifecycle_state = EXCLUDED.lifecycle_state,
        context_payload = EXCLUDED.context_payload,
        proposed_action = EXCLUDED.proposed_action,
        updated_at = CURRENT_TIMESTAMP
    WHERE agent_feed_items.lifecycle_state IS DISTINCT FROM EXCLUDED.lifecycle_state
       OR agent_feed_items.context_payload IS DISTINCT FROM EXCLUDED.context_payload
       OR agent_feed_items.proposed_action IS DISTINCT FROM EXCLUDED.proposed_action;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_agent_feed_sync_items ON agent_feed;
CREATE TRIGGER trg_agent_feed_sync_items
AFTER INSERT OR UPDATE ON agent_feed
FOR EACH ROW
EXECUTE FUNCTION sync_agent_feed_to_items();

CREATE OR REPLACE FUNCTION sync_agent_feed_delete()
RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM agent_feed_items WHERE id = OLD.id;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_agent_feed_sync_delete ON agent_feed;
CREATE TRIGGER trg_agent_feed_sync_delete
AFTER DELETE ON agent_feed
FOR EACH ROW
EXECUTE FUNCTION sync_agent_feed_delete();

CREATE OR REPLACE FUNCTION sync_items_to_agent_feed()
RETURNS TRIGGER AS $$
BEGIN
    IF pg_trigger_depth() > 1 THEN
        RETURN NEW;
    END IF;
    UPDATE agent_feed
    SET state = NEW.lifecycle_state,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id
      AND state IS DISTINCT FROM NEW.lifecycle_state;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_items_sync_agent_feed ON agent_feed_items;
CREATE TRIGGER trg_items_sync_agent_feed
AFTER UPDATE OF lifecycle_state ON agent_feed_items
FOR EACH ROW
EXECUTE FUNCTION sync_items_to_agent_feed();
