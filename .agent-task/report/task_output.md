outcome: blocked
issue_title: Architect Native Rust Omnichannel Chat to Replace Chatwoot
issue_description: |
  Blocked/No-Work finding: The requested feature to "Architect Native Rust Omnichannel Chat to Replace Chatwoot" (Issue #35945) has already been implemented and is fully covered by existing tests.

  1. **Schema Requirements (`Inbox`, `Conversation`, `Message`, `Contact` with PostgreSQL RLS/tenant isolation):**
     Already implemented via migrations in `src/server/migrations/` and SeaORM entities in `src/server/integrations/omnichannel/src/models.rs`. The tables (`chat_inboxes`, `chat_conversations`, `chat_messages`, `chat_contacts`) all enforce `tenant_id` policies.
  2. **Core Domain and Service Layer:**
     Already implemented in `src/server/domain/chat/mod.rs` (defining core models) and `src/server/integrations/omnichannel/src/repository.rs` (defining `ChatRepository` and `InMemoryChatRepository`).
  3. **Channel Adapters and Routing:**
     Already implemented in `src/server/integrations/omnichannel/src/traits.rs` (`ChannelAdapter`) and `src/server/integrations/omnichannel/src/router.rs` (`MessageRouter`).
  4. **Test Coverage:**
     The `server_integrations_omnichannel` test suite fully covers these models, the in-memory repository, routing, and channel adapters. I ran `cargo test -p server_integrations_omnichannel` and all 8 tests passed successfully:
     - `tests::test_contact_model_creation ... ok`
     - `tests::test_conversation_model_creation ... ok`
     - `tests::test_inbox_model_creation ... ok`
     - `tests::test_message_model_creation ... ok`
     - `tests::test_in_memory_repository ... ok`
     - `tests::test_message_router_creates_contact_and_conversation ... ok`
     - `tests::test_message_router_reuses_existing_conversation ... ok`
     - `tests::test_mock_channel_adapter ... ok`

  Since all requirements requested in the issue are demonstrably present and passing their relevant tests, no code changes are necessary or required in this scope.
