-- +goose Up
ALTER TABLE bookings ADD COLUMN IF NOT EXISTS ai_summary TEXT;

-- +goose Down
ALTER TABLE bookings DROP COLUMN IF EXISTS ai_summary;
