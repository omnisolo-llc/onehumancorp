-- 159_add_consumable_order_fields.sql
-- Adds the `is_consumable` and `estimated_duration_days` fields to the `orders` table.

ALTER TABLE orders
ADD COLUMN is_consumable BOOLEAN DEFAULT FALSE,
ADD COLUMN estimated_duration_days INT DEFAULT NULL;
