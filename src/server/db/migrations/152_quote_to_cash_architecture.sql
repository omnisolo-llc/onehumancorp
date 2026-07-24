-- +goose Up
-- Add missing columns for Quote-to-Cash architecture
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS linked_invoice_id UUID;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS calendar_event_id TEXT;
ALTER TABLE quotes ADD COLUMN IF NOT EXISTS deposit_payment_intent_id TEXT;

-- We need to update the constraint on `status` to allow the new states
-- Postgres doesn't allow dropping a check constraint if we don't know its name directly,
-- so we'll query it and execute dynamically, or just add a new check constraint that's broader.
-- Better approach: just drop the existing check constraint dynamically.
DO $$
DECLARE
    con_name text;
BEGIN
    SELECT constraint_name INTO con_name
    FROM information_schema.check_constraints
    WHERE constraint_name LIKE '%quotes_status_check%';

    IF con_name IS NOT NULL THEN
        EXECUTE 'ALTER TABLE quotes DROP CONSTRAINT ' || con_name;
    END IF;
END $$;

ALTER TABLE quotes ADD CONSTRAINT quotes_status_check
CHECK (status IN ('DRAFT', 'PENDING_APPROVAL', 'SENT', 'ACCEPTED', 'REJECTED', 'EXPIRED', 'DEPOSIT_PAID', 'WORK_COMPLETED', 'FULLY_PAID'));

-- +goose Down
ALTER TABLE quotes DROP COLUMN IF EXISTS linked_invoice_id;
ALTER TABLE quotes DROP COLUMN IF EXISTS calendar_event_id;
ALTER TABLE quotes DROP COLUMN IF EXISTS deposit_payment_intent_id;

DO $$
DECLARE
    con_name text;
BEGIN
    SELECT constraint_name INTO con_name
    FROM information_schema.check_constraints
    WHERE constraint_name LIKE '%quotes_status_check%';

    IF con_name IS NOT NULL THEN
        EXECUTE 'ALTER TABLE quotes DROP CONSTRAINT ' || con_name;
    END IF;
END $$;

ALTER TABLE quotes ADD CONSTRAINT quotes_status_check
CHECK (status IN ('DRAFT', 'PENDING_APPROVAL', 'SENT', 'ACCEPTED', 'REJECTED', 'EXPIRED'));
