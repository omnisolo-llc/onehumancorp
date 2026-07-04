-- Drop the single UNIQUE constraint on idempotency_key if it exists and replace it with a composite unique constraint per tenant
ALTER TABLE payment_intents DROP CONSTRAINT IF EXISTS payment_intents_idempotency_key_key;
ALTER TABLE payment_intents ADD CONSTRAINT payment_intents_tenant_id_idempotency_key_key UNIQUE (tenant_id, idempotency_key);
