-- Canonical actor-owned phone proof. Legacy process-global settings are never imported.
CREATE TABLE IF NOT EXISTS sms_notification_preferences (
    tenant_id TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    phone TEXT,
    verification_id TEXT,
    current_challenge_id TEXT,
    urgent_booking BOOLEAN NOT NULL DEFAULT FALSE,
    failed_payment BOOLEAN NOT NULL DEFAULT FALSE,
    new_order BOOLEAN NOT NULL DEFAULT FALSE,
    send_window BIGINT NOT NULL DEFAULT 0,
    send_count INTEGER NOT NULL DEFAULT 0,
    last_requested_at BIGINT NOT NULL DEFAULT 0,
    updated_at BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (tenant_id, actor_id),
    CHECK ((phone IS NULL AND verification_id IS NULL AND NOT urgent_booking AND NOT failed_payment AND NOT new_order)
        OR (phone IS NOT NULL AND verification_id IS NOT NULL)),
    CHECK (send_count >= 0)
);
CREATE TABLE IF NOT EXISTS sms_verification_challenges (
    tenant_id TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    challenge_id TEXT NOT NULL,
    phone TEXT NOT NULL,
    code_mac TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('sending','accepted','unknown','rejected','verified','superseded')),
    provider_sid TEXT,
    created_at BIGINT NOT NULL,
    expires_at BIGINT NOT NULL,
    attempts INTEGER NOT NULL DEFAULT 0 CHECK (attempts BETWEEN 0 AND 5),
    PRIMARY KEY (tenant_id, actor_id, challenge_id),
    CHECK (expires_at > created_at),
    CHECK (state NOT IN ('accepted','verified') OR provider_sid IS NOT NULL),
    FOREIGN KEY (tenant_id, actor_id) REFERENCES sms_notification_preferences(tenant_id,actor_id)
);
CREATE TABLE IF NOT EXISTS sms_notification_events (
    tenant_id TEXT NOT NULL,
    event_id TEXT NOT NULL,
    event_type TEXT NOT NULL CHECK (event_type IN ('urgent_booking','failed_payment','new_order')),
    message TEXT NOT NULL,
    message_hash TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('prepared','no_recipients','provider_accepted','no_eligible_recipients')),
    created_at BIGINT NOT NULL,
    PRIMARY KEY (tenant_id, event_id, event_type)
);
CREATE TABLE IF NOT EXISTS sms_notification_dispatches (
    tenant_id TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    event_id TEXT NOT NULL,
    event_type TEXT NOT NULL CHECK (event_type IN ('urgent_booking','failed_payment','new_order')),
    phone TEXT NOT NULL,
    verification_id TEXT NOT NULL,
    message_hash TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('prepared','sending','accepted','unknown','rejected','cancelled')),
    provider_sid TEXT,
    created_at BIGINT NOT NULL,
    PRIMARY KEY (tenant_id, actor_id, event_id, event_type),
    CHECK (state != 'accepted' OR provider_sid IS NOT NULL)
);
ALTER TABLE sms_notification_preferences ENABLE ROW LEVEL SECURITY;
ALTER TABLE sms_notification_preferences FORCE ROW LEVEL SECURITY;
ALTER TABLE sms_verification_challenges ENABLE ROW LEVEL SECURITY;
ALTER TABLE sms_verification_challenges FORCE ROW LEVEL SECURITY;
ALTER TABLE sms_notification_dispatches ENABLE ROW LEVEL SECURITY;
ALTER TABLE sms_notification_dispatches FORCE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_sms_notification_preferences ON sms_notification_preferences
USING (tenant_id = current_setting('app.current_tenant',true))
WITH CHECK (tenant_id = current_setting('app.current_tenant',true));
CREATE POLICY tenant_isolation_sms_verification_challenges ON sms_verification_challenges
USING (tenant_id = current_setting('app.current_tenant',true))
WITH CHECK (tenant_id = current_setting('app.current_tenant',true));
CREATE POLICY tenant_isolation_sms_notification_dispatches ON sms_notification_dispatches
USING (tenant_id = current_setting('app.current_tenant',true))
WITH CHECK (tenant_id = current_setting('app.current_tenant',true));
ALTER TABLE sms_notification_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE sms_notification_events FORCE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_sms_notification_events ON sms_notification_events
USING (tenant_id = current_setting('app.current_tenant',true))
WITH CHECK (tenant_id = current_setting('app.current_tenant',true));
