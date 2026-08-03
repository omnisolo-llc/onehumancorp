-- Test file for native omnichannel chat migration
BEGIN;

-- Test ENUMs
SELECT 'open'::conversation_status;
SELECT 'sent'::message_status;

-- Test tables exist and have tenant_id
SELECT tenant_id FROM inboxes LIMIT 0;
SELECT tenant_id FROM contacts LIMIT 0;
SELECT tenant_id FROM conversations LIMIT 0;
SELECT tenant_id FROM chat_messages LIMIT 0;

-- Test RLS is enabled
SELECT relrowsecurity FROM pg_class WHERE relname = 'inboxes';
SELECT relrowsecurity FROM pg_class WHERE relname = 'contacts';
SELECT relrowsecurity FROM pg_class WHERE relname = 'conversations';
SELECT relrowsecurity FROM pg_class WHERE relname = 'chat_messages';

-- Test UUID v7 Generation
SELECT generate_uuid_v7();

ROLLBACK;
