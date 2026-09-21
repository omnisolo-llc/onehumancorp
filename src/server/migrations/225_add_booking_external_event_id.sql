-- Add external_event_id to bookings table to link OHC bookings with external calendar events
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name='bookings' AND column_name='external_event_id') THEN
        ALTER TABLE bookings ADD COLUMN external_event_id TEXT;
    END IF;
END $$;
