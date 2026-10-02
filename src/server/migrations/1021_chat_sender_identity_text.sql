-- Chat senders are polymorphic: contacts may use UUIDs, while authenticated
-- accounts use opaque text IDs. Preserve every existing UUID spelling without
-- assigning a synthetic identity or adding a user-only foreign key.
-- PostgreSQL preserves/rebuilds compatible indexes and rejects incompatible
-- dependent constraints atomically; do not drop constraints with CASCADE.
ALTER TABLE chat_messages
    ALTER COLUMN sender_id TYPE TEXT USING sender_id::text;
