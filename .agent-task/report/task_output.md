outcome: no_work
issue_title: "Architecture: Native Rust Omnichannel Chat System (Omnichannel Replacement)"
issue_description: |
  Superpowers skill provenance: None loaded

  The request asks to implement the native Rust omnichannel chat system to replace the legacy system, including the `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` tables with RLS enabled, as well as a gRPC/REST API and frontend components for the unified inbox list and conversation view, and E2E Playwright tests demonstrating an owner reading a drafted AI reply and approving it.

  Upon investigating the codebase, I verified that this feature is already complete:
  - The PostgreSQL schema with RLS is already implemented in `src/server/migrations/1009_native_omnichannel_chat.sql` and `src/server/migrations/1001_create_omni_inbox_messages_and_quotes_fix.sql`.
  - The native Rust backend service models and operations are implemented in `src/server/services/chat/models.rs` and `src/server/services/chat/service.rs`.
  - The omnichannel webhook functionality is present in `src/server/api/omnichannel_webhook.rs`.
  - The frontend inbox list and conversation view exist in `src/ui/next/src/app/inbox/page.tsx` (using Next.js/React instead of Flutter, which aligns with the repository's stack).
  - Playwright E2E tests for the AI drafted reply and approval flow are already written in `src/e2e/playwright/omnichannel_approval.mock-contract.ts` and `src/ui/next/src/e2e/omni_inbox_triage.spec.ts`.

  Since the feature is fully implemented and aligned with the existing technology stack, there is no further work required.
