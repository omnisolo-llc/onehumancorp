-- +goose Up
-- Migration 176: Add subscription churn prediction fields

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='subscribers' AND column_name='health_score') THEN
        ALTER TABLE subscribers ADD COLUMN health_score INTEGER DEFAULT 100;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='subscribers' AND column_name='last_health_check_at') THEN
        ALTER TABLE subscribers ADD COLUMN last_health_check_at TIMESTAMPTZ;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='subscribers' AND column_name='churn_risk_status') THEN
        ALTER TABLE subscribers ADD COLUMN churn_risk_status TEXT DEFAULT 'healthy';
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    ALTER TABLE subscribers DROP COLUMN IF EXISTS health_score;
    ALTER TABLE subscribers DROP COLUMN IF EXISTS last_health_check_at;
    ALTER TABLE subscribers DROP COLUMN IF EXISTS churn_risk_status;
END
$$;
