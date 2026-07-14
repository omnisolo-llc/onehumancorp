-- +goose Up
-- Add failed_reason to ohc_job_queue
ALTER TABLE ohc_job_queue ADD COLUMN IF NOT EXISTS failed_reason TEXT;

-- +goose Down
-- Reverting RLS changes
ALTER TABLE ohc_job_queue DROP COLUMN IF EXISTS failed_reason;
