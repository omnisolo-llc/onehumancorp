issue_title: Implement Native Rust Omnichannel Chat System Core Architecture
issue_description: "The Native Rust Omnichannel Chat System is already fully implemented. Migrations 233_chat_omnichannel.sql provide the exact schema including Row Level Security (RLS) for chat_inboxes, chat_channels, chat_contacts, chat_conversations, and chat_messages. The Chatwoot-parity models and service logic are implemented in src/server/services/chat/ and src/server/domain/chat/. The corresponding gRPC and REST endpoints are already in src/server/api/widget/chat.rs. We experienced test failures with cargo check and test and timeouts with make test-backend."
issue_priority: P1
issue_category: Infrastructure
issue_type: Feature
issue_label: chat
assignees: []
outcome: no_work
