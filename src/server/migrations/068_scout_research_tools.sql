-- Seed the researched tools into the database
INSERT INTO tool_integrations (tenant_id, name, description, api_url, status)
VALUES
    ('system', 'Ayrshare', 'Unified Social Media Inbox and Cross-Posting', 'https://api.ayrshare.com/v1', 'pending'),
    ('system', 'Cal.com', 'Zero-Config Booking & Calendar Sync', 'https://api.cal.com/v1', 'pending'),
    ('system', 'Listmonk', 'Embedded, No-Jargon Email Campaigns', 'http://localhost:9000/api', 'pending'),
    ('system', 'Mercado Pago', 'Expand Payments with Mercado Pago for LATAM Users', 'https://api.mercadopago.com/v1', 'pending'),
    ('system', 'EasyPost', 'Painless Shipping Labels & Tracking', 'https://api.easypost.com/v2', 'pending'),
    ('system', 'Twilio', 'Global SMS Alerts & Customer Notifications', 'https://api.twilio.com/2010-04-01', 'pending'),
    ('system', 'Jitsi Meet', 'Zero-Setup Online Lessons', 'https://meet.jit.si', 'pending')
ON CONFLICT DO NOTHING;
