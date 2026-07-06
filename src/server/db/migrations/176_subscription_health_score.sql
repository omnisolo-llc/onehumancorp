-- +goose Up
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='subscribers' AND column_name='health_score') THEN
        ALTER TABLE subscribers ADD COLUMN health_score INTEGER DEFAULT 100;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='subscribers' AND column_name='last_activity_date') THEN
        ALTER TABLE subscribers ADD COLUMN last_activity_date TIMESTAMPTZ;
    END IF;
END
$$;
