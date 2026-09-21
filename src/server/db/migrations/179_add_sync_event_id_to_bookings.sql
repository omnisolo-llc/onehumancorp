-- +goose Up
ALTER TABLE bookings ADD COLUMN IF NOT EXISTS sync_event_id TEXT;

-- +goose Down
ALTER TABLE bookings DROP COLUMN IF EXISTS sync_event_id;
