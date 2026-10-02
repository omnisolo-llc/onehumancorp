outcome: no_work
issue_title: "[36825] Native Rust Omnichannel Chat System Architecture"
issue_description: |
  **Verified Task Context:**
  The issue requests the implementation of a Native Rust Omnichannel Chat System Architecture for OneHumanCorp, specifically replacing Chatwoot with native Rust domain models, PostgreSQL RLS, and an internal API for inboxes, channels, contacts, conversations, and messages.

  **Verification Results:**
  Exploration of the codebase revealed that the Native Rust Omnichannel Chat System Architecture has already been implemented:
  - **Database Schema:** `src/server/db/migrations/1009_native_omnichannel_chat.sql` defines the required tables (`chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, `chat_messages`) with `tenant_id` and strict Row-Level Security (RLS) policies enforcing `app.current_tenant_id`.
  - **Domain Models:** `src/server/domain/chat/mod.rs` fully implements the domain models (`Inbox`, `Contact`, `Conversation`, `Message`) in Rust with 100% unit test coverage provided.
  - **Internal API & Services:** `src/server/services/chat/service.rs` implements the backend API and database integration using SQLx, providing methods like `create_inbox`, `create_channel`, `create_contact`, `start_conversation`, and `send_message`, which also enforce the RLS via `SET LOCAL app.current_tenant_id`. It includes comprehensive unit tests testing creation and isolation boundaries.

  Since the system requested by the issue's acceptance criteria has already been verified as implemented in the current code, this issue is a blocked no-work finding.
