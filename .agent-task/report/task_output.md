outcome: no_work
issue_title: Implement Custom Rust Omnichannel Chat System
issue_description: The native Rust omnichannel chat system is already fully implemented in the codebase. Existing files like src/server/integrations/omnichannel/src/models.rs, src/server/services/chat/service.rs, and the database migrations (src/server/db/migrations/1009_native_omnichannel_chat.sql) contain the required tables (chat_inboxes, chat_conversations, chat_messages) with row-level security enabled and all requested gRPC/REST APIs implemented in Rust.
