outcome: no_work
issue_title: Implement Chatwoot-Compatible Core Omnichannel Models in Rust
issue_description: |
  The requested omnichannel models are already fully implemented in the codebase. Comparing against the issue's acceptance criteria:
  1. The database migrations for inboxes, contacts, conversations, and messages exist in src/server/db/migrations/1009_native_omnichannel_chat.sql with strict tenant_id Row Level Security (RLS) policies.
  2. The Rust structs and repository interfaces to perform CRUD operations are implemented in src/server/services/chat/models.rs and src/server/services/chat/service.rs.
  3. Tenant isolation via RLS is unit-tested in the test_rls_isolation function within src/server/services/chat/service.rs.
  4. The repositories are integrated into the existing backend service layer as ChatService.
  All acceptance criteria are satisfied, and there are no unverified criteria.
