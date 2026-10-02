outcome: no_work
issue_title: 'Implement Custom Rust Omnichannel Chat System to Replace Chatwoot'
issue_description: 'The requested feature to implement a custom Rust omnichannel chat system replacing Chatwoot is already implemented. The migration 1009_native_omnichannel_chat.sql sets up the database schema for chat_inboxes and chat_channels with RLS tenant isolation. The migration 233_chat_omnichannel.sql sets up chat_contacts. The Rust services for CRUD operations exist in src/server/services/chat/service.rs and use sqlx. The required real-time infrastructure is part of src/server/api/unified_ws.rs. The UI scaffold exists in src/ui/next/src/app/inbox/page.tsx. There are no Chatwoot dependencies or code found in the repository. Unverified criteria: 100% unit tests, E2E tests, sea-orm entities, chat_conversations table, chat_messages table, and chat_contacts RLS isolation.'
issue_priority: P0
issue_category: Swarm Category: IMPLEMENTER
issue_type: 'no-work finding'
issue_label: 'feature'
assignees: []
