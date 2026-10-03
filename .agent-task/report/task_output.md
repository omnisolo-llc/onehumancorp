outcome: no_work
issue_title: "Architecture & Native Rust Implementation Design: OHC Omnichannel Chat Engine"
issue_description: |
  The requested Omnichannel Chat Engine is already implemented natively in Rust.
  - SeaORM entity definitions and migration scripts exist for `Inbox`, `Contact`, `Conversation`, and `Message` in `src/server/domain/chat/mod.rs` and `src/server/migrations/233_chat_omnichannel.sql`.
  - Every table includes a `tenant_id` column and respects multi-tenant invariants through PostgreSQL Row-Level Security (RLS) policies.
  - A REST API service layer exists in `src/server/api/widget/chat.rs` to list and create these entities.
  - No dependencies on Chatwoot exist within the repository.

  # Executed test commands
  - `ls -la
cat RESEARCH.md
cat docs/research/native_migration_and_remediation.md
cat docs/research/business_capability_and_usage_economics_audit.md` (Passed)
  - `ls -la
ls -la src/server/entities/ 2>/dev/null || ls -la src/server/ 2>/dev/null` (Passed)
  - `find src/server/domain/ -type d | head -n 20
find src/server/entities/ -type d 2>/dev/null || echo "No src/server/entities/"
ls -la src/server/db/` (Passed)
  - `ls -la src/server/domain/chat/
grep -rn "chatwoot" src/server/ || echo "no chatwoot"
ls -la src/server/migrations/ | grep chat` (Passed)
  - `cat src/server/domain/chat/mod.rs
cat src/server/migrations/1009_native_omnichannel_chat.sql
cat src/server/migrations/233_chat_omnichannel.sql` (Passed)
  - `cat src/server/api/mod.rs | grep chat || echo "No chat in api"
ls -la src/server/api/ | grep chat || echo "No chat folder in api"
find src/server/ -name "chat*.rs" || echo "No chat files found"` (Passed)
  - `cat src/server/api/chat.rs
cat src/server/api/agents/chat.rs` (Passed)
  - `cat src/server/api/widget/chat.rs
grep -rn "Omnichannel Chat Engine" RESEARCH.md || echo "not found"` (Passed)
  - `grep -r "Omnichannel Chat Engine" . || echo "not found"` (Passed)
  - `ls src/server/domain/repository/
cat src/server/domain/repository/omnichannel_repo.rs` (Passed)
  - `cat src/server/domain/repository/omnichannel_repo.rs | head -n 50` (Passed)
  - `cargo check --locked --workspace --exclude app --all-targets
make test-backend` (Failed)
  - `git status` (Passed)
