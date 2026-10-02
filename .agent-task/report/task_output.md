outcome: no_work
issue_title: 'Implement Native Rust Omnichannel Chat Engine (Chatwoot Replacement)'
issue_description: |
  The native Rust omnichannel chat engine (Chatwoot replacement) is already implemented in the codebase.

  Comparing the issue's acceptance criteria against the current implementation:
  * Database migrations for inboxes, channels, contacts, conversations, and messages (including tenant_id isolation via PostgreSQL RLS) are verified in src/server/migrations/233_chat_omnichannel.sql.
  * Rust CRUD services using sqlx are verified in src/server/domain/repository/omnichannel_repo.rs.
  * A webhook ingestion endpoint for channels is verified in src/server/services/omnichannel_service.rs (ingest_signal).

  The implementation exists. Therefore, this request requires no new implementation.
