outcome: no_work
issue_title: "Architecture Design: Native Rust Multi-Tenant Omnichannel Chat Engine"
issue_description: |
  The requested feature—a native Rust multi-tenant omnichannel chat engine utilizing Axum, Tokio, and Redis Pub/Sub—is already implemented in the current codebase.

  **Evidence from current implementation:**
  - **Omnichannel Data Models and Tables:** `ChatInbox`, `ChatChannel`, `ChatContact`, `ChatConversation`, and `ChatMessage` are fully defined as Rust structs with `tenant_id` fields in `src/server/services/chat/models.rs` and `src/server/integrations/omnichannel/src/models.rs`.
  - **Database Migrations and RLS:** Migrations like `src/server/migrations/1009_native_omnichannel_chat.sql` create the chat tables and explicitly enforce PostgreSQL Row-Level Security via `CREATE POLICY ... ON chat_... FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);`.
  - **WebSocket and Real-time Messaging:** The existing unified WebSocket endpoint defined in `src/server/api/unified_ws.rs` already provides tenant-scoped pub/sub over Redis using Axum WebSockets (`handle_unified_socket`). It authenticates users, resolves the `tenant_id`, and manages subscriptions per tenant.
  - **Tenant Isolation Tests:** `scripts/chat-tenant-isolation/run.sh` contains extensive tenant-context regression tests explicitly designed to ensure isolation across the chat data access boundaries.
  - **Chatwoot Replacement:** According to `docs/reports/production_agent_optimization_report.md` ("CHAT-00 — Chatwoot removal"), Chatwoot has already been entirely removed and replaced with the native omnichannel inbox architecture.

  Because all requirements stated in the problem description are already satisfied by the codebase, no further implementation is necessary for this issue.

assignees: []
