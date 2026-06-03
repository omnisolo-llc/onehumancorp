-- Consolidate subscription schemas

DROP TABLE IF EXISTS fulfillment_batches CASCADE;
DROP TABLE IF EXISTS subscribers CASCADE;

-- If needed, drop the old subscription_plans table so the one from 018_zero_touch_subscriptions.sql takes precedence, or alter it.
-- But wait, 018_subscriptions.sql runs. We can just DROP TABLE subscription_plans CASCADE?
-- Let's just drop them and let the application recreate them, or rely on 018_zero_touch_subscriptions.sql.
