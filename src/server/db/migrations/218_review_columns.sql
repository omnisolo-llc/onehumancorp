-- Add platform, sentiment, and published_at to reviews table
ALTER TABLE reviews ADD COLUMN IF NOT EXISTS platform TEXT DEFAULT 'OHC';
ALTER TABLE reviews ADD COLUMN IF NOT EXISTS sentiment TEXT;
ALTER TABLE reviews ADD COLUMN IF NOT EXISTS published_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;
