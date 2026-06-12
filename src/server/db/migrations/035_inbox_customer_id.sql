-- +goose Up
ALTER TABLE inbox_messages
ADD COLUMN customer_id TEXT;

ALTER TABLE omni_inbox_messages
ADD COLUMN customer_id TEXT;

