outcome: no_work
issue_title: "GitHub Issue #36609: Architect Native Rust Omnichannel Inbox (Chatwoot Replacement)"
issue_description: |
  The requested Native Rust Omnichannel Inbox features are already fully implemented in the current stack.
  - Rust API backend endpoints exist in src/server/api/inbox/webhook.rs and src/server/api/inbox_api.rs.
  - Database schema models for multi-tenant omnichannel inboxes exist in src/server/db/migrations/1009_native_omnichannel_chat.sql and 031_c_omni_inbox_messages.sql.
  - The UI for the 375px mobile-first inbox view is built in src/ui/next/src/app/inbox/page.tsx (using the repo's Next.js standard instead of Flutter).
  - Playwright E2E tests validating this functionality exist in src/e2e/omnichannel_unified_inbox.mock-contract.ts and src/e2e/omni_inbox_webhook.mock-contract.ts.
