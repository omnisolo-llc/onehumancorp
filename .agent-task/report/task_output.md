outcome: no_work
issue_title: "Architect Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  **Verified Task Status:** The Native Rust Omnichannel Chat System (Chatwoot Replacement) has already been fully implemented in the existing codebase.

  **Evidence of Completion:**
  - **Database Migrations:** The exact required PostgreSQL tables (`chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, `chat_messages`) with Row Level Security (RLS) enforcement and indexes have already been implemented in `src/server/migrations/233_chat_omnichannel.sql`.
  - **Data Models:** The corresponding Rust struct models (`ChatInbox`, `ChatChannel`, `ChatContact`, `ChatConversation`, `ChatMessage`) with SQLx FromRow mapping are fully implemented in `src/server/services/chat/models.rs`.
  - **Service Layer / SQLx Methods:** The `ChatService` containing all required basic CRUD operations (e.g., `create_inbox`, `create_channel`, `create_contact`, `start_conversation`, `send_message`) is fully implemented in `src/server/services/chat/service.rs`.
  - **Tests / Acceptance Criteria:** Complete tenant isolation and multi-tenancy testing using RLS and transaction scoping is explicitly covered in `src/server/services/chat/service.rs` under the `test_rls_isolation` and `test_create_inbox` test functions. Zero dependencies on Chatwoot exist (Chatwoot removal has already been documented and validated in previous tasks such as `docs/superpowers/plans/2026-07-13-chatwoot-removal.md`).

  Therefore, no further implementation work is necessary as the exact requirements detailed in the issue are already present and verified in the source code.
