outcome: no_work
issue_title: "Issue #36619: [Native Chat] Core Rust Omnichannel Inbox & Conversation Architecture"
issue_description: |
  The requested core database schema and foundational Rust backend models/CRUD operations for the native OHC Omnichannel Inbox are already implemented.
  Migrations `1009_native_omnichannel_chat.sql` and `233_chat_omnichannel.sql` define the required tables with RLS policies.
  Rust data models and basic repository methods (create inbox, start conversation, send message) are fully implemented in `src/server/services/chat/models.rs` and `src/server/services/chat/service.rs`.
  Test coverage and cross-tenant data leakage tests exist (e.g. `test_rls_isolation`).
issue_priority: P1
issue_category: Engineering
issue_type: Feature
issue_label: [agent-report]
assignees: []
