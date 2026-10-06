-- The canonical SQLite orders schema is also installed by DB::new. Auth-only
-- startup must install the same table, never a reduced incompatible shadow.
CREATE TABLE IF NOT EXISTS orders (
    id TEXT PRIMARY KEY,
    tenant_id TEXT,
    customer_id TEXT,
    total_amount REAL,
    currency TEXT DEFAULT 'USD',
    status TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_subscribable BOOLEAN DEFAULT FALSE,
    subscription_frequency TEXT,
    subscription_discount_percent INTEGER DEFAULT 0,
    _sync_status TEXT DEFAULT 'pending',
    version INTEGER DEFAULT 1,
    is_consumable BOOLEAN NOT NULL DEFAULT FALSE,
    estimated_duration_days INTEGER
);

-- Admission is in the business INSERT transaction. No historical backfill.
CREATE TRIGGER IF NOT EXISTS orders_admit_sms AFTER INSERT ON orders
WHEN NEW.tenant_id IS NOT NULL AND length(NEW.tenant_id) BETWEEN 1 AND 512
 AND trim(NEW.tenant_id)=NEW.tenant_id AND lower(NEW.tenant_id)!='system'
 AND length(NEW.id) BETWEEN 1 AND 255 AND trim(NEW.id)=NEW.id
 AND instr(NEW.id,char(0))=0 AND NEW.id NOT GLOB ('*['||char(1)||'-'||char(31)||char(127)||'-'||char(159)||']*')
 AND NOT EXISTS(SELECT 1 FROM sms_notification_events e WHERE e.tenant_id=NEW.tenant_id AND e.event_id=NEW.id AND e.event_type='new_order')
BEGIN
    INSERT INTO sms_notification_events(tenant_id,event_id,event_type,message,message_hash,status,created_at)
    VALUES(NEW.tenant_id,NEW.id,'new_order','A new order has been saved. Open OmniSolo to review it.',
        'ea077c658f64495269b8e6b40d71658395387f2470d40038bdac298f01abcebd','prepared',unixepoch());
    INSERT INTO sms_notification_dispatches(tenant_id,actor_id,event_id,event_type,phone,verification_id,message_hash,state,created_at)
    SELECT p.tenant_id,p.actor_id,NEW.id,'new_order',p.phone,p.verification_id,
        'ea077c658f64495269b8e6b40d71658395387f2470d40038bdac298f01abcebd','prepared',unixepoch()
    FROM sms_notification_preferences p
    JOIN users u ON u.id=p.actor_id AND u.tenant_id=p.tenant_id
    JOIN sms_verification_challenges c ON c.tenant_id=p.tenant_id AND c.actor_id=p.actor_id
        AND c.challenge_id=p.verification_id AND c.phone=p.phone
    WHERE p.tenant_id=NEW.tenant_id AND u.active=TRUE AND p.new_order=TRUE AND c.state='verified'
        AND EXISTS(SELECT 1 FROM identity_user_roles r WHERE r.user_id=u.id AND r.tenant_id=u.tenant_id AND lower(r.role_name) IN ('owner','admin'));
    UPDATE sms_notification_events SET status='no_recipients'
    WHERE tenant_id=NEW.tenant_id AND event_id=NEW.id AND event_type='new_order'
        AND NOT EXISTS(SELECT 1 FROM sms_notification_dispatches d WHERE d.tenant_id=NEW.tenant_id AND d.event_id=NEW.id AND d.event_type='new_order');
END;
CREATE INDEX IF NOT EXISTS sms_order_outbox_pending ON sms_notification_events(event_type,status,next_attempt_at,created_at,tenant_id,event_id);
