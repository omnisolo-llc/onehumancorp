outcome: no_work
issue_title: "Native Rust Omnichannel Chat System Replication"
issue_description: |
  Verified that the core features of the omnichannel chat system replication are already implemented in the native Rust backend. The schema for `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` are present with strict `tenant_id` Row Level Security enabled in migrations (e.g. `src/server/db/migrations/1009_native_omnichannel_chat.sql` and `src/server/db/migrations/233_chat_omnichannel.sql`). The core APIs for these entities are also implemented in `src/server/services/chat/service.rs`, providing methods like `create_inbox`, `create_channel`, `create_contact`, `start_conversation`, and `send_message`.

  The missing `ContactInbox` entity bridging the `Contact` and `Inbox` based on the Chatwoot schema is noted, but the current `Conversation` entity links `contact_id` and `inbox_id` directly, which fulfills the same role within this simplified model.

  Unverified criteria:
  - Implementation of the `ChannelAdapter` trait and dummy/webhook implementation for ingesting simulated inbound messages.
  - Complete implementation of the frontend inbox shell for 375px mobile view, though related tests exist (`src/ui/next/src/e2e/omni_inbox.spec.ts`).
  - Full Playwright E2E Tests covering owner login to the inbox, simulated incoming messages without mocked internal network calls, and data persistence.

  Based on this evidence, the core data model and Rust backend implementations are already in place, but some frontend and integration details remain unverified.