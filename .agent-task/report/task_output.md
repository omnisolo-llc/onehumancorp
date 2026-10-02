issue_title: "OHC AI Capabilities & Competitor Audit: Enhancing the Owner Work Assistant"
issue_description: |
  The request asks to implement the foundation for a native Rust omnichannel chat engine to replace Chatwoot, including core data models (Conversation, Message), API for unified inbox, Work Triage AI integration, and 375px responsive UI with "Approve/Edit" action buttons.

  Verification of the repository state reveals that all acceptance criteria are already satisfied:
  - The native Rust chat core models (Conversation, Message) are already implemented in `src/server/services/chat/models.rs` and `src/server/services/chat/service.rs`.
  - The PostgreSQL schema has been successfully migrated (`src/server/migrations/1009_native_omnichannel_chat.sql`) with tables `chat_conversations` and `chat_messages`.
  - The unified inbox and "Work Triage" AI integration exist in `src/server/services/inbox/service.rs` (using `UnifiedThread`, `UnifiedMessage`, and `UnifiedTriageAction`).
  - The UI for the unified inbox with the AI summary and "Approve & Send Draft" / action buttons is fully implemented in `src/ui/next/src/app/inbox/page.tsx` and supports responsive mobile viewports.
  - Chatwoot was previously removed (as tracked in `docs/reports/production_agent_optimization_report.md` under CHAT-00).

  Since the requested feature is already fully present in the codebase, no safe, well-scoped implementation follows from this task. Any new implementation would result in duplicate architectural components.
issue_priority: "P0"
issue_category: "ui"
issue_type: "agent-report"
issue_label: "ohc:lane:ui"
assignees: []
outcome: "no_work"
