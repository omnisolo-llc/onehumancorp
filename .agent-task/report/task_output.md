outcome: no_work
issue_title: "Implement AI Unified Inbox & Agent Feed Differentiation"
issue_description: |
  The requested feature "Implement AI Unified Inbox & Agent Feed Differentiation" (Issue #34546) is already fully implemented in the codebase.

  Evidence:
  - Webhook ingestion and Identity Resolution are implemented in src/server/api/inbox/webhook.rs and src/server/api/inbox/identity.rs.
  - The AI Agent drafting (Ambassador Agent logic) is implemented in src/server/workers/message_triage_worker.rs, which uses OmniContextRouter to analyze messages and draft context-aware replies (e.g., Draft Reply, Draft Quote).
  - The generated drafts are placed into the action queues (updating omni_inbox_messages, and inserting into triage_proposed_actions / agent_feed_items).
  - The Mobile-first 375px UI for the unified inbox with the "1-Tap Approve" button is implemented in src/ui/next/src/app/inbox/page.tsx.
  - Playwright E2E tests already exist to verify this flow, including src/e2e/tests/omni_inbox_differentiation.mock-contract.ts and src/e2e/playwright/omnichannel_unified_inbox.mock-contract.ts.

  Loaded skills: skills/using-superpowers/SKILL.md
  Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
