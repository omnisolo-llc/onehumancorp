CREATE TABLE IF NOT EXISTS video_tutorials (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    url TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS help_articles (
    id TEXT PRIMARY KEY,
    category TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO video_tutorials (id, title, description, url)
VALUES
    ('1', 'How to add your first product', 'A quick 60-second guide to listing items in your store.', 'https://example.com/video1'),
    ('2', 'Setting up AI Helpers', 'Learn how to let AI handle your customer emails and social media.', 'https://example.com/video2')
ON CONFLICT (id) DO NOTHING;

INSERT INTO help_articles (id, category, title, description)
VALUES
    ('1', 'Getting Started', 'Set up your store in 5 minutes', 'Follow our simple guide to add your first product and go live.'),
    ('2', 'My Store', 'How to add products', 'Learn how to list new items, add photos, and set prices.'),
    ('3', 'Payments & Billing', 'How to accept Apple Pay', 'Enable Apple Pay with one click in your payment settings.'),
    ('4', 'AI Helpers', 'What can the Customer Success Helper do?', 'Your helper can reply to customer emails and Instagram DMs automatically.'),
    ('5', 'Marketing', 'How to run a promotion', 'Learn how to create discount codes and share them on social media.'),
    ('6', 'Account & Billing', 'How to change your subscription', 'Find out how to upgrade or downgrade your plan and view past invoices.')
ON CONFLICT (id) DO NOTHING;
