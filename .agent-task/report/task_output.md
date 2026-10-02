outcome: blocked
issue_title: "[Architecture] Native Rust Omnichannel Inbox & Chat System"
issue_description: |
  The issue requested replacing an external Chatwoot dependency with a native Rust implementation, which involves adding several database tables, webhook ingestion, and real-time processing components. However, upon inspection, the Chatwoot dependency was already removed on 2026-07-13, and its removal is heavily documented and verified by regression scripts (`deploy/tests/no_chatwoot_residue_test.sh`). More importantly, a native Omnichannel inbox implementation *already exists* under `omni_inbox_messages`, `chat_messages` (via `1009_native_omnichannel_chat.sql`), and related UI components (`src/ui/next/src/app/inbox/page.tsx`). The feature is already built. No code changes are required.
issue_priority: "P0"
issue_category: "architecture"
issue_type: "feature"
issue_label: ""
assignees: []
