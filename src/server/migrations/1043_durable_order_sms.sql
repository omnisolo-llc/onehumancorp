-- Reuse the reviewed SMS event/audience outbox at the canonical order commit.
-- This function is SECURITY INVOKER: it neither bypasses RLS nor grants authority.
-- The INSERT and its frozen event/audience commit or roll back together. Existing
-- orders are intentionally not backfilled, so upgrades never send old alerts.
ALTER TABLE sms_notification_events ADD COLUMN next_attempt_at BIGINT NOT NULL DEFAULT 0;

CREATE OR REPLACE FUNCTION admit_order_sms() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF NEW.tenant_id IS NULL OR length(NEW.tenant_id) NOT BETWEEN 1 AND 512
       OR btrim(NEW.tenant_id)<>NEW.tenant_id OR lower(NEW.tenant_id)='system'
       OR length(NEW.id) NOT BETWEEN 1 AND 255 OR btrim(NEW.id)<>NEW.id OR NEW.id ~ '[[:cntrl:]]' THEN
        RETURN NEW;
    END IF;
    IF EXISTS(SELECT 1 FROM sms_notification_events e WHERE e.tenant_id=NEW.tenant_id AND e.event_id=NEW.id AND e.event_type='new_order') THEN
        RETURN NEW;
    END IF;
    INSERT INTO sms_notification_events(tenant_id,event_id,event_type,message,message_hash,status,created_at)
    VALUES(NEW.tenant_id,NEW.id,'new_order','A new order has been saved. Open OmniSolo to review it.',
        'ea077c658f64495269b8e6b40d71658395387f2470d40038bdac298f01abcebd','prepared',extract(epoch FROM clock_timestamp())::bigint);
    -- Materialize a single eligibility snapshot while locking the same current
    -- identity/roles/preferences as dispatch. Proof itself is locked as well.
    WITH recipients AS MATERIALIZED (
        SELECT p.tenant_id,p.actor_id,p.phone,p.verification_id
        FROM sms_notification_preferences p
        JOIN users u ON u.id=p.actor_id AND u.tenant_id=p.tenant_id
        JOIN identity_user_roles r ON r.user_id=u.id AND r.tenant_id=u.tenant_id
        JOIN sms_verification_challenges c ON c.tenant_id=p.tenant_id AND c.actor_id=p.actor_id
            AND c.challenge_id=p.verification_id AND c.phone=p.phone
        WHERE p.tenant_id=NEW.tenant_id AND u.active=TRUE AND p.new_order=TRUE
            AND c.state='verified' AND lower(r.role_name) IN ('owner','admin')
        ORDER BY p.actor_id
        FOR SHARE OF u,r,c FOR UPDATE OF p
    )
    INSERT INTO sms_notification_dispatches(tenant_id,actor_id,event_id,event_type,phone,verification_id,message_hash,state,created_at)
    SELECT DISTINCT tenant_id,actor_id,NEW.id,'new_order',phone,verification_id,
        'ea077c658f64495269b8e6b40d71658395387f2470d40038bdac298f01abcebd','prepared',extract(epoch FROM transaction_timestamp())::bigint
    FROM recipients;
    UPDATE sms_notification_events SET status='no_recipients'
    WHERE tenant_id=NEW.tenant_id AND event_id=NEW.id AND event_type='new_order'
        AND NOT EXISTS(SELECT 1 FROM sms_notification_dispatches d WHERE d.tenant_id=NEW.tenant_id AND d.event_id=NEW.id AND d.event_type='new_order');
    RETURN NEW;
END;
$$;
CREATE TRIGGER orders_admit_sms AFTER INSERT ON orders FOR EACH ROW EXECUTE FUNCTION admit_order_sms();
CREATE INDEX IF NOT EXISTS sms_order_outbox_pending ON sms_notification_events(event_type,status,next_attempt_at,created_at,tenant_id,event_id);
