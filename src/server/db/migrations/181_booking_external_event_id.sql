-- +goose Up
ALTER TABLE bookings ADD COLUMN external_event_id TEXT;
-- +goose Down
ALTER TABLE bookings DROP COLUMN external_event_id;
