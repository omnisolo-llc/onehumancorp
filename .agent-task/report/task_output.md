issue_title: Implement Native Rust Omnichannel Chat System
issue_description: "The issue requests the implementation of a native Rust omnichannel chat system to replace Chatwoot, including strict tenant isolation (RLS), AI agent integration, and a mobile-first Flutter UI.\n\nAfter researching the repository and reading the audit/remediation ledger and architecture docs (e.g., `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`, `RESEARCH.md`), I found that the native omnichannel chat system has already been implemented in Rust. The codebase contains `OmniChannelService`, `OmniChannelRepo`, `MessageRouter`, `ChatRepository`, and the associated core data entities (`chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, `chat_messages`) with PostgreSQL RLS enabled for `tenant_id` (via migrations like `233_chat_omnichannel.sql` and `1009_native_omnichannel_chat.sql`). The Next.js frontend has also replaced Flutter for the mobile-first UI, as explicitly stated in the `RESEARCH.md` and `README.md` ('The old Slint/Flutter UI is also removed; the maintained UI is Next.js inside the Tauri shell or a Node web deployment.'). Chatwoot has been fully removed (verified by absence of Chatwoot integration files, docker services, helm charts, etc.).\n\nTherefore, the requested functionality is already implemented and the repository constraints override the legacy instruction to build a Flutter UI. No new work is needed."
issue_priority: P0
issue_category: backend
issue_type: feature
issue_label: []
assignees: []
outcome: no_work
