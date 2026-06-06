-- Migration 077: Unified Product and Booking Data Model

-- Add duration column for services/bookings to products table
ALTER TABLE products ADD COLUMN IF NOT EXISTS duration_minutes INT DEFAULT NULL;

-- For unified onboarding, we want to allow an initial prompt or setup configuration
-- to be saved onto the business. We will add an onboarding_prompt column to the businesses table.
ALTER TABLE businesses ADD COLUMN IF NOT EXISTS onboarding_prompt TEXT;
