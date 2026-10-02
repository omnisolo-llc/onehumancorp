outcome: no_work
issue_title: 'Implement Custom Rust Omnichannel Chat System to Replace Chatwoot'
issue_description: |
  The core functionality requested in the issue (implementing a native Rust omnichannel chat system within onehumancorp/mono to replace Chatwoot) is already implemented and verified in the current codebase.
  - The Chatwoot integration has been entirely removed from the application, as evidenced by docs/reports/production_agent_optimization_report.md (CHAT-00 - Chatwoot removal).
  - Core database schema and data models with tenant isolation (tenant_id) exist in src/server/domain/repository/omnichannel_repo.rs and src/server/services/chat/models.rs (ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage).
  - A native Rust backend routing logic and API endpoints for managing inboxes, contacts, and conversations are implemented (src/server/services/chat/service.rs, src/server/services/omnichannel_service.rs, src/server/api/unified_inbox_webhook.rs, src/server/api/widget/chat.rs).
  - AI Assistant drafting integration is implemented, utilizing LLM for context-aware drafting based on business context (src/server/services/omnichannel_service.rs and src/server/services/inbox/service.rs).
  - The Web Widget channel is supported (src/server/api/widget/chat.rs).
  - Real-time WebSocket delivery is present in src/server/api/unified_ws.rs and related modules.
  - A unified mobile-first UI for the Team Chat and Action Feed is available (src/ui/next/src/app/team/chat/page.tsx and src/ui/next/src/app/team/page.tsx).
  - End-to-end (E2E) Playwright tests exist for the unified chat flows (e.g., src/e2e/ui/help_center.spec.ts, src/e2e/team_chat.spec.ts, src/e2e/tests/zero_click_onboarding.spec.ts).
