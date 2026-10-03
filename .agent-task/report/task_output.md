outcome: no_work
issue_title: Native Rust Omnichannel Chat System & Chatwoot Retirement
issue_description: |
  The requested task is to implement the foundational native Rust data models and API for the OHC Omnichannel Chat system, completely replacing external Chatwoot dependencies.

  However, upon auditing the repository, this work has already been completed.

  Evidence:
  1. The docs/reports/production_agent_optimization_report.md file contains a section "CHAT-00 — Chatwoot removal" which states:
     "Status (2026-07-13): Removed from the active application and deployment graph."
  2. The src/server/services/chat/models.rs and src/server/services/chat/service.rs files already contain the native Rust implementation for ChatInbox, ChatChannel, ChatContact, ChatConversation, and ChatMessage.
  3. The database migrations for creating these tables and enforcing Row-Level Security (RLS) already exist (e.g., src/server/db/migrations/233_chat_omnichannel.sql).
  4. The required Superpowers workflow was followed, loading skills/using-superpowers/SKILL.md from upstream commit 8ca22dba9a94f28898bbce59f2537ff4d87c747d.

  As the work described in the issue is already complete, no further changes are necessary.
