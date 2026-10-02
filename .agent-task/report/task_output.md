outcome: no_work
issue_title: "🛠️ Taskmaster: [Native Omnichannel Chat & Web Widget Platform already implemented]"
issue_description: |
  I have verified the repository for Issue #36656 regarding Native Omnichannel Chat & Web Widget Platform.
  The work requested is already complete in the current codebase:
  1. **Schema Migrations**: `src/server/migrations/233_chat_omnichannel.sql` contains the tables (`chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, `chat_messages`) and enables RLS on `tenant_id`.
  2. **Rust Domain Models & Repo Layer**: Implemented via `ChatService` inside `src/server/services/chat/service.rs` handling DB interactions, and `OmniChannelRepo` inside `src/server/domain/repository/omnichannel_repo.rs`.
  3. **API Endpoints**: `src/server/api/widget/chat.rs` fully defines endpoints for creating conversations and messages (`create_conversation`, `create_message`, `get_messages`).
  4. **Widget Snippet**: Files like `chat-embed.html` present in `src/ui/next/public` fulfill the embed requirements.
  5. **Unit Tests**: Coverage exists within the backend module (e.g., `src/server/services/chat/service.rs`).
  6. **E2E Tests**: Several tests verify this behavior, including `src/e2e/omnichannel_unified_inbox.mock-contract.ts` and `src/e2e/omni_inbox.spec.ts`.

  Since the required features are fully implemented, this is marked as a no-work finding.
issue_priority: P0
issue_category: Architecture
issue_type: Implementation
issue_label: omnichannel
assignees: []
