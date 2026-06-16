-- +goose Up
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS last_follow_up_at TIMESTAMPTZ;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS follow_up_count INTEGER DEFAULT 0;

-- +goose Down
ALTER TABLE quotes DROP COLUMN IF EXISTS last_follow_up_at;
ALTER TABLE quotes DROP COLUMN IF EXISTS follow_up_count;
