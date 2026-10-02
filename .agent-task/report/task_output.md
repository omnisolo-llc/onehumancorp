outcome: blocked
issue_title: "[blocked no-work finding: Native Rust Omnichannel Chat System]"
issue_description: |
  **Verified Task Target:** Implement core PostgreSQL data models and foundational Rust service layer for the OHC Native Omnichannel Chat system (GitHub Issue #36910).

  **Findings:**
  The core components of the Native Omnichannel Chat system described in the issue are already implemented.

  *   **Database Migrations:** The required migration `1009_native_omnichannel_chat.sql` exists in `src/server/db/migrations/` and accurately creates the `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` tables. It includes the mandatory `tenant_id` columns, sets up PostgreSQL Row-Level Security (RLS), and defines proper isolation policies.
  *   **Rust Models:** The corresponding SQLx struct models (`ChatInbox`, `ChatChannel`, `ChatContact`, `ChatConversation`, `ChatMessage`) exist in `src/server/services/chat/models.rs` using UUIDs and standard timestamps.
  *   **Service Layer:** Core CRUD operations exist in `src/server/services/chat/service.rs`. `ChatService` handles creating conversations.

  Therefore, the core task to "implement the core PostgreSQL data models and foundational Rust service layer" is already complete. No further safe, well-scoped implementation follows from the current repository state for this issue.

  **Unverified Acceptance Criteria:**
  Full compilation checks block on unrelated compilation errors in the core `omnisolo` crate (specifically `src/server/integrations/registry.rs`) due to duplicated fields (`google_workspace_clients`) and missing values (`creds`). This prevents running `bazel test //...` or `make test` over the entire repository effectively. I ran a targeted `cargo test -q -p server_harness --test omnisolo_live` instead to verify repository state.
