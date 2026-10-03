outcome: blocked
issue_title: "Architecture: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  Blocked no-work finding:
  The requested omnichannel unified inbox UI and backend systems already exist in the codebase, and the Chatwoot replacement strategy is already satisfied by the native implementation.

  Code evidence:
  - `src/server/migrations/233_chat_omnichannel.sql` and `1009_native_omnichannel_chat.sql` define the requested `chat_inboxes`, `chat_conversations`, `chat_messages`, etc.
  - `src/server/api/omnichannel_webhook.rs` implements the backend event ingestion, resolving customer identities and inserting into the native inbox tables.
  - `src/ui/next/src/app/inbox/page.tsx` contains the omnichannel inbox UI, complete with filtering and message rendering.
  - `src/ui/next/src/e2e/omni_inbox_triage.spec.ts` provides E2E tests for the native inbox flow, asserting the presence of the inbox, the mocked messages (e.g. 'Do you have vegan options'), and the generated action buttons.

  Because the native Rust API, data models, Playwright tests, and Next.js frontend elements (rather than the explicitly forbidden Flutter Dart) for the omnichannel unified inbox are already present, no further implementation is necessary.
issue_priority: P0
issue_category: architecture
issue_type: research
issue_label: agent-report
assignees: []
