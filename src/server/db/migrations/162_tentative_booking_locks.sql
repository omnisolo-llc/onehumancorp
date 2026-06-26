-- +goose Up
ALTER TABLE estimates ADD COLUMN IF NOT EXISTS proposed_slot_id TEXT REFERENCES booking_slots(id);
ALTER TABLE estimates ADD COLUMN IF NOT EXISTS locked_slot_start TIMESTAMPTZ;
ALTER TABLE estimates ADD COLUMN IF NOT EXISTS locked_slot_end TIMESTAMPTZ;

ALTER TABLE booking_slots ADD COLUMN IF NOT EXISTS soft_locked_by TEXT;

-- +goose Down
ALTER TABLE booking_slots DROP COLUMN IF EXISTS soft_locked_by;

ALTER TABLE estimates DROP COLUMN IF EXISTS proposed_slot_id;
ALTER TABLE estimates DROP COLUMN IF EXISTS locked_slot_start;
ALTER TABLE estimates DROP COLUMN IF EXISTS locked_slot_end;
