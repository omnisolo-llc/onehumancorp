-- +goose Up
-- Add abandoned status to service_leads
ALTER TABLE service_leads DROP CONSTRAINT IF EXISTS service_leads_status_check;
ALTER TABLE service_leads ADD CONSTRAINT service_leads_status_check CHECK (status IN ('new', 'estimating', 'estimated', 'booked', 'closed', 'abandoned'));

-- Add abandoned status to estimates
ALTER TABLE estimates DROP CONSTRAINT IF EXISTS estimates_status_check;
ALTER TABLE estimates ADD CONSTRAINT estimates_status_check CHECK (status IN ('draft', 'sent', 'approved', 'rejected', 'expired', 'abandoned'));

-- +goose Down
ALTER TABLE estimates DROP CONSTRAINT IF EXISTS estimates_status_check;
ALTER TABLE estimates ADD CONSTRAINT estimates_status_check CHECK (status IN ('draft', 'sent', 'approved', 'rejected', 'expired'));

ALTER TABLE service_leads DROP CONSTRAINT IF EXISTS service_leads_status_check;
ALTER TABLE service_leads ADD CONSTRAINT service_leads_status_check CHECK (status IN ('new', 'estimating', 'estimated', 'booked', 'closed'));
