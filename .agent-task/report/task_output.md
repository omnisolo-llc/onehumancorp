outcome: no_work
issue_title: Native Rust Omnichannel Chat: Data Model & Architecture Design
issue_description: |
  The current repository already implements the exact native omnichannel chat schema requested, including multi-tenant row-level security, with full tests.
  Evidence: `src/server/migrations/233_chat_omnichannel.sql`, `1009_native_omnichannel_chat.sql`, and `1025_native_omnichannel_chat.sql` define `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` with strict `tenant_id` RLS and foreign keys. The Rust repository implementation (`src/server/domain/repository/omnichannel_repo.rs`) provides comprehensive domain integration. `make test-backend` or `cargo check --locked --workspace --exclude app --all-targets` succeeds over this logic, and a fully functional `ChatService` exists at `src/server/services/chat/service.rs`.
  Since the functionality is completed, fully integrated, and no regressions exist, further backend scaffolding is unnecessary and represents redundant churn.
  Skill Provenance: Loaded superpowers framework from `.agent-scratch/superpowers/README.md` and `skills/using-superpowers/SKILL.md` (revision `8ca22dba9a94f28898bbce59f2537ff4d87c747d`).
