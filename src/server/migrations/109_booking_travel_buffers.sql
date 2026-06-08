-- Add travel_buffer_minutes to services table for booking padding
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name='services' AND column_name='travel_buffer_minutes') THEN
        ALTER TABLE services ADD COLUMN travel_buffer_minutes INTEGER NOT NULL DEFAULT 0;
    END IF;
END $$;
