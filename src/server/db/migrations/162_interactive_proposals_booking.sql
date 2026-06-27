-- +goose Up
ALTER TABLE interactive_proposals ADD COLUMN IF NOT EXISTS service_id TEXT;
ALTER TABLE interactive_proposals ADD COLUMN IF NOT EXISTS proposed_slot_id TEXT;

-- +goose Down
ALTER TABLE interactive_proposals DROP COLUMN IF EXISTS proposed_slot_id;
ALTER TABLE interactive_proposals DROP COLUMN IF EXISTS service_id;
