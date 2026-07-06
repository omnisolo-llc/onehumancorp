-- +goose Up
-- Add health_score and last_engagement_at to subscriptions
DO $$
BEGIN
    IF to_regclass('subscriptions') IS NOT NULL THEN
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='subscriptions' AND column_name='health_score') THEN
            ALTER TABLE subscriptions ADD COLUMN health_score INTEGER DEFAULT 100;
        END IF;
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='subscriptions' AND column_name='last_engagement_at') THEN
            ALTER TABLE subscriptions ADD COLUMN last_engagement_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;
        END IF;
    END IF;
END
$$;

-- Add health_score and last_engagement_at to subscribers
DO $$
BEGIN
    IF to_regclass('subscribers') IS NOT NULL THEN
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='subscribers' AND column_name='health_score') THEN
            ALTER TABLE subscribers ADD COLUMN health_score INTEGER DEFAULT 100;
        END IF;
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='subscribers' AND column_name='last_engagement_at') THEN
            ALTER TABLE subscribers ADD COLUMN last_engagement_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;
        END IF;
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    IF to_regclass('subscriptions') IS NOT NULL THEN
        ALTER TABLE subscriptions DROP COLUMN IF EXISTS health_score;
        ALTER TABLE subscriptions DROP COLUMN IF EXISTS last_engagement_at;
    END IF;
    IF to_regclass('subscribers') IS NOT NULL THEN
        ALTER TABLE subscribers DROP COLUMN IF EXISTS health_score;
        ALTER TABLE subscribers DROP COLUMN IF EXISTS last_engagement_at;
    END IF;
END
$$;
