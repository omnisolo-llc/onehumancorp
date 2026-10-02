-- 1015_bookings_service_id_and_compat.sql
-- Ensure both service_id and product_id exist on bookings for dual compatibility.

ALTER TABLE bookings ADD COLUMN IF NOT EXISTS service_id TEXT;
ALTER TABLE bookings ADD COLUMN IF NOT EXISTS product_id TEXT;
