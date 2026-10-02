outcome: blocked
issue_title: "OHC AI Capabilities & Competitor Audit: Enhancing the Owner Work Assistant"
issue_description: |
  The current repository natively supports an omnichannel chat engine and unified triage interface. The required feature described in the issue (replacing Chatwoot with a native Rust chat engine, supporting 375px mobile triage, and AI-generated Draft Reply) is already implemented and verified in the codebase.

  Acceptance criteria satisfied:
  1. `Conversation`/`Message` core data models are present in `src/server/services/chat/models.rs` and `chat_conversations` table is defined in SQL.
  2. The webhook/API infrastructure (`handle_unified_webhook` in `src/server/api/unified_inbox_webhook.rs`) exists to handle incoming messages, simulating the requested incoming DM logic.
  3. The AI Work Triage integration uses `src/server/domain/repository/agent_feed_repo.rs`, identifying the `Instagram DM` intent and proposing actions (e.g., Draft Reply).
  4. The 375px native frontend UI (`src/ui/next/src/app/triage/page.tsx`) displays the actionable feed, including draft replies, priority status, and 'Approve & Send' features.

  Because the requested architecture and features already exist, there are no meaningful new code changes to safely implement.
