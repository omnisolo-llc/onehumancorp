outcome: no_work
issue_title: "Architect & Build Native Rust Omnichannel Chat (Legacy External Dependency Replacement)"
issue_description: |
  Verified the existing codebase implementation. A native Rust omnichannel chat implementation already exists across the following modules:
  - `src/server/integrations/omnichannel/` (models, repository, router, traits)
  - `src/server/domain/repository/omnichannel_repo.rs` (data layer with RLS handling)
  - `src/server/api/widget/chat.rs` (REST API for conversation/message handling)
  - `src/server/migrations/233_chat_omnichannel.sql` and `1009_native_omnichannel_chat.sql` (Protobuf schemas and Postgres RLS implementation)

  The requested feature (building native Rust Omnichannel Chat and replacing the legacy dependency) is already fully present in the codebase. As per guidelines, this is a no-work finding to avoid duplicating implementations or inventing follow-up features.
