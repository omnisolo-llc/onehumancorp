CREATE TABLE IF NOT EXISTS manual_inbox_intents (
    tenant_id TEXT NOT NULL,
    inbox_message_id TEXT NOT NULL,
    revision BIGINT NOT NULL CHECK (revision > 0),
    PRIMARY KEY (tenant_id,inbox_message_id)
);
-- Explicit signed-user requests are separate from agent approvals. Routing,
-- author and content are frozen once; only pending ownership can be retired.
CREATE TABLE IF NOT EXISTS manual_inbox_requests (
    tenant_id TEXT NOT NULL,
    request_id TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    inbox_message_id TEXT NOT NULL,
    dispatch_id TEXT NOT NULL,
    source TEXT NOT NULL,
    recipient TEXT NOT NULL,
    body TEXT NOT NULL,
    provider_binding TEXT NOT NULL,
    payload_hash TEXT NOT NULL,
    intent TEXT NOT NULL CHECK (intent IN ('send','dismissed','resolved')),
    intent_revision BIGINT NOT NULL CHECK (intent_revision > 0),
    state TEXT NOT NULL CHECK (state IN ('pending','claimed','retired','dismissed','resolved')),
    created_at BIGINT NOT NULL,
    expires_at BIGINT NOT NULL,
    PRIMARY KEY (tenant_id,request_id),
    UNIQUE (tenant_id,dispatch_id)
);
CREATE UNIQUE INDEX IF NOT EXISTS manual_inbox_pending_owner
ON manual_inbox_requests(tenant_id,inbox_message_id) WHERE state='pending';
ALTER TABLE manual_inbox_requests ENABLE ROW LEVEL SECURITY;
ALTER TABLE manual_inbox_requests FORCE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_manual_inbox_requests ON manual_inbox_requests
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
CREATE FUNCTION reject_manual_inbox_identity_edit() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION 'manual request identity is immutable';
END $$;
CREATE TRIGGER manual_inbox_request_immutable
BEFORE UPDATE OF tenant_id,request_id,actor_id,inbox_message_id,dispatch_id,source,recipient,body,provider_binding,payload_hash,intent,intent_revision,created_at ON manual_inbox_requests
FOR EACH ROW EXECUTE FUNCTION reject_manual_inbox_identity_edit();
ALTER TABLE manual_inbox_intents ENABLE ROW LEVEL SECURITY;
ALTER TABLE manual_inbox_intents FORCE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_manual_inbox_intents ON manual_inbox_intents
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
