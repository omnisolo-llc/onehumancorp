-- +goose Up
ALTER TABLE inbox_messages
ADD COLUMN customer_id TEXT;

ALTER TABLE omni_inbox_messages
ADD COLUMN customer_id TEXT;

-- +goose Down
ALTER TABLE inbox_messages
DROP COLUMN customer_id;

ALTER TABLE omni_inbox_messages
DROP COLUMN customer_id;
