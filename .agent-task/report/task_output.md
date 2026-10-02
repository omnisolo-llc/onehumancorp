outcome: no_work
issue_title: "[Architect] Implement Native Rust Omnichannel Chat System"
issue_description: |
  I have investigated the codebase and found that the requested feature (Native Rust Omnichannel Chat System) is already fully implemented, as part of the codebase native migration and the existing `chat` service and DB schemas.

  * The PostgreSQL database tables (`chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, `chat_messages`) with Row Level Security (RLS) are created in `src/server/db/migrations/1009_native_omnichannel_chat.sql` and `src/server/db/migrations/233_chat_omnichannel.sql`.
  * The native Axum backend service that implements the logic is located at `src/server/services/chat/service.rs`. It provides the ability to create inboxes, channels, contacts, and start conversations.
  * The Playwright E2E tests, including owner approval flow, are located in `src/ui/next/src/e2e/omni_inbox.spec.ts`, `src/ui/next/src/e2e/approval_inbox.spec.ts` and others.
  * The Mobile UX flow and approval elements (`Approve & Send Draft`) are implemented in `src/ui/next/src/app/inbox/page.tsx`.

  Therefore, no new work is required for this issue. I have not added any dummy changes.
