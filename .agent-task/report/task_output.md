outcome: no_work
issue_title: "[Native Chat] Implement Rust-Native Omnichannel Models & Schema"
issue_description: "The native omnichannel chat schema and models are already implemented. The migration 1009_native_omnichannel_chat.sql and 233_chat_omnichannel.sql already establish the required PostgreSQL tables with tenant_id and RLS (chat_inboxes, chat_channels, chat_contacts, chat_conversations, chat_messages). Furthermore, the Rust models and basic CRUD operations (with RLS isolation tests) are already defined in src/server/services/chat/models.rs and src/server/services/chat/service.rs."
