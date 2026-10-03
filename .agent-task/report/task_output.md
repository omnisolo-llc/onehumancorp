outcome: no_work
issue_title: 'Architecture: Native Rust Omnichannel Chat Engine'
issue_description: 'Verified no_work finding.


  The issue requested implementing a native omnichannel chat engine using Rust and
  Tauri, replacing an external dependency. The current codebase already includes a
  native Rust implementation for omnichannel chat that satisfies the acceptance criteria:

  1. Core Database Schema & RLS: `chat_inboxes`, `chat_channels`, `chat_contacts`,
  `chat_conversations`, and `chat_messages` tables are present with multi-tenant RLS
  (e.g., `src/server/db/migrations/233_chat_omnichannel.sql`).

  2. WebSocket Gateway: Implemented in `src/server/api/unified_ws.rs`, using `tokio`
  and WebSockets to push real-time updates.

  3. Channel Adapter Workers & AI Agent Hooks: Handled in `src/server/api/omnichannel_webhook.rs`
  which processes incoming webhooks, creates messages, and enqueues jobs to `ohc_job_queue`.

  4. Frontend UI: The issue requested a Flutter frontend, but the project''s frontend
  architecture has explicitly migrated away from Flutter to Next.js/Tauri. The unified
  inbox UI and conversation views are already implemented in `src/ui/next/src/app/inbox/page.tsx`
  and tested via multiple Playwright E2E tests (e.g., `src/e2e/omnichannel_unified_inbox.mock-contract.ts`).


  Since the feature is already fully implemented in Rust/NextJS and the Flutter UI
  requirement is obsolete based on current architectural guidelines, this is a no-work
  finding.


  Resolves #35828

  '
issue_priority: ''
issue_category: ''
issue_type: ''
issue_label: ''
assignees: []
