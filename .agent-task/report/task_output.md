outcome: no_work
issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  The native omnichannel chat system is already fully implemented in the current codebase.

  Verified the following against the acceptance criteria:
  - Database schema: `chat_inboxes`, `chat_contacts`, `chat_conversations`, and `chat_messages` exist with Row-Level Security (RLS) in `src/server/db/migrations/233_chat_omnichannel.sql`.
  - API and WebSocket handlers: Bi-directional real-time message syncing is implemented in `src/server/api/realtime.rs`, and omnichannel webhooks are processed in `src/server/api/omni_inbox_webhook.rs`.
  - Internal API layer: `src/server/services/chat/service.rs` handles internal logic for creating messages, while `src/server/domain/repository/omnichannel_repo.rs` manages conversation history.
  - UI interface: `src/ui/next/src/app/inbox/page.tsx` implements the mobile-first inbox and conversation view, including "Approve & Send Draft" logic.
  - E2E Tests: `src/e2e/playwright/omnichannel_unified_inbox.mock-contract.ts` verifies a real-time message received via webhook and the subsequent triage approval.

  As all requirements specified in the design doc for Issue #35738 are already satisfied by the existing implementation, this task is marked as no-work.
issue_priority: P0
issue_category: backend
issue_type: feature
issue_label: agent-report
assignees: []
