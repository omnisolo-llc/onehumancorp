-- NULL identifies legacy rows without a verifiable request receipt. Never backfill.
ALTER TABLE ohc_timecard_event ADD COLUMN IF NOT EXISTS request_identity TEXT;
