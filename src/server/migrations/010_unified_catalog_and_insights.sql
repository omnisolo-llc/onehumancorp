-- Migration: Unified Catalog and AI Insights
-- Description: Adds tables for unified catalog items, order drafts, and weekly insights.

CREATE TABLE IF NOT EXISTS unified_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    price_cents BIGINT NOT NULL,
    type TEXT NOT NULL CHECK (type IN ('PHYSICAL', 'DIGITAL', 'SERVICE')),
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    duration_minutes INTEGER,
    image_url TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS order_drafts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    source_channel TEXT NOT NULL,
    raw_message TEXT NOT NULL,
    suggested_amount_cents BIGINT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS weekly_insights (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    action_label TEXT NOT NULL,
    type TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Seed Data for Fatima (Maya) the Baker
INSERT INTO weekly_insights (organization_id, title, description, action_label, type)
VALUES
('00000000-0000-0000-0000-000000000000', 'Sales are slow this week', 'Your strawberry cakes had 50 views but 0 sales. Should I draft an Instagram post with a 10% discount?', 'Yes, Do It', 'MARKETING');

INSERT INTO order_drafts (organization_id, source_channel, raw_message, suggested_amount_cents)
VALUES
('00000000-0000-0000-0000-000000000000', 'WhatsApp', 'Can I get 2 dozen cupcakes for Friday?', 4000);
