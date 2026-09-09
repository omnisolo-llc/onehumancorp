-- Keep PostgreSQL products aligned with the inventory API and the standalone
-- schema. These columns previously existed only in the legacy migration tree.

ALTER TABLE products
    ADD COLUMN IF NOT EXISTS is_subscribable BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE products
    ADD COLUMN IF NOT EXISTS subscription_frequency TEXT;
ALTER TABLE products
    ADD COLUMN IF NOT EXISTS subscription_discount_percent INTEGER NOT NULL DEFAULT 0;

UPDATE products SET is_subscribable = FALSE WHERE is_subscribable IS NULL;
UPDATE products
SET subscription_discount_percent = 0
WHERE subscription_discount_percent IS NULL;

ALTER TABLE products ALTER COLUMN is_subscribable SET DEFAULT FALSE;
ALTER TABLE products ALTER COLUMN is_subscribable SET NOT NULL;
ALTER TABLE products ALTER COLUMN subscription_discount_percent SET DEFAULT 0;
ALTER TABLE products ALTER COLUMN subscription_discount_percent SET NOT NULL;
