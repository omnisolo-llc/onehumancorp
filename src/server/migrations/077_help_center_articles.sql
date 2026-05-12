CREATE TABLE IF NOT EXISTS help_center_articles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id TEXT NOT NULL,
    category TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT idx_help_center_org_title UNIQUE (organization_id, title)
);

ALTER TABLE help_center_articles ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Tenant isolation for help_center_articles" ON help_center_articles
    USING (organization_id = current_setting('app.current_tenant', true)::text OR organization_id = 'system');

INSERT INTO help_center_articles (organization_id, category, title, description) VALUES
('system', 'Getting Started', 'Set up your store in 5 minutes', 'Follow our simple guide to add your first product and go live.'),
('system', 'My Store', 'How to add products', 'Learn how to list new items, add photos, and set prices so your customers can start buying today.'),
('system', 'Payments', 'How to accept Apple Pay', 'Enable Apple Pay with one click in your payment settings.'),
('system', 'AI Agents', 'What can the Customer Success Helper do?', 'Your AI helper can reply to customer emails and Instagram DMs automatically, saving you hours of work each week.'),
('system', 'Marketing', 'How to run a promotion', 'Learn how to create discount codes and share them on social media.'),
('system', 'Troubleshooting', 'App is running slow', 'Learn how to clear temporary files and speed up the app so you can get back to running your business.'),
('system', 'Account & Billing', 'How to change your subscription', 'Find out how to upgrade or downgrade your plan and view past invoices.')
ON CONFLICT (organization_id, title) DO NOTHING;
