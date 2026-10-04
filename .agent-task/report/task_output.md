outcome: no_work
issue_title: "Research: AI Assistant for Unified Customer Inbox & Communications"
issue_description: |
  The requested feature to implement a Unified Customer Inbox using a Go backend and a Flutter mobile UI is blocked as a no-work finding. The repository's current technology stack is Rust and Next.js, not Go and Flutter.

  Furthermore, a review of the current codebase reveals that the functionality described in the issue (a unified inbox for omnichannel messages with AI-drafted replies and approval workflows) is already implemented natively in the current stack:
  - The data model (`omni_inbox_messages`) exists in the PostgreSQL schema with multi-tenant RLS (e.g., `src/server/migrations/146_omni_inbox_rls.sql`).
  - The backend endpoints and logic for handling these messages, drafting replies, and tracking status are present in `src/server/lib.rs`.
  - The Next.js frontend UI (`src/ui/next/src/app/inbox/page.tsx`) already provides the unified inbox view, displaying the customer context, message, AI draft reply, and an "Approve & Send Draft" button, fully satisfying the requested Critical User Journey (CUJ).
  - E2E tests covering the Omni-Inbox flow also exist (e.g., `src/e2e/omni_inbox.mock-contract.ts`).

  Since the feature is already implemented and the requested technology stack is obsolete, no further work is required.
