issue_title: "[Implement Omnichannel Chat Data Models and Message Router]"
issue_description: |
  # Native Rust Omnichannel Chat: Architecture & Data Models

  Implemented the native Rust omnichannel chat engine data models and message router according to the design specifications in #36671. The implemented components include:
  - `ChatRepository` trait and `InMemoryChatRepository` implementation with async multi-tenant constraints.
  - `MessageRouter` that resolves/creates `contact` and `conversation` entities on incoming messages.
  - Core models definitions (`contact`, `conversation`, `inbox`, `message`) equipped with SeaORM derives and multi-tenant `tenant_id` fields.
  - `ChannelAdapter` trait system to receive webhooks and send messages.
  - Tests covering model creation, routing behavior, and repository data retention in `src/server/integrations/omnichannel`.

  Executed test commands:
  - `ls -la` (Passed)
  - `mkdir -p .scratch/superpowers && cd .scratch/superpowers && git init && git remote add origin https://github.com/obra/superpowers.git && git fetch --depth 1 origin main && git checkout FETCH_HEAD` (Passed)
  - `cat .scratch/superpowers/skills/using-superpowers/SKILL.md` (Passed)
  - `cat RESEARCH.md` (Truncated output)
  - `cat README.md` (Truncated output)
  - `ls -la src/server/` (Passed)
  - `ls -la src/server/integrations/chat` (Failed)
  - `ls -la src/server/integrations/` (Passed)
  - `ls -la src/server/integrations/omnichannel/` (Passed)
  - `ls -la src/server/integrations/omnichannel/src/` (Passed)
  - `cat src/server/integrations/omnichannel/src/models.rs` (Passed)
  - `cat src/server/integrations/omnichannel/src/traits.rs` (Passed)
  - `cat src/server/integrations/omnichannel/src/lib.rs` (Passed)
  - `cat src/server/integrations/omnichannel/Cargo.toml` (Passed)
  - `cd src/server/integrations/omnichannel && cargo test --locked` (Passed)
  - `grep -r "test_conversation_model_creation" src/server/integrations/omnichannel/` (Passed)
  - `grep -ri contact src/server/integrations/omnichannel/` (Passed)
  - `cat Cargo.toml | grep tokio` (Passed)
  - `cd src/server/integrations/omnichannel && cargo test --locked` (Failed)
  - `make test-backend` (Failed)
  - `rm -rf .scratch/superpowers` (Passed)
  - `ls -la .scratch` (Passed)
  - `mkdir -p src/server/integrations/omnichannel/src` (Passed)
  - `cargo fmt --all` (Passed)
  - `cd src/server/integrations/omnichannel && cargo test --locked` (Passed)
  - `cargo clippy -p server_integrations_omnichannel --all-targets -- -D warnings` (Passed)

  # Verified trace limitations
  - `cat RESEARCH.md` was truncated.
  - `cat README.md` was truncated.
  - `cd src/server/integrations/omnichannel && cargo test --locked` (the first one) had truncated output (although passed).
  - `make test-backend` timed out (recorded as Failed).
  - Full workspace `make lint` and `cargo check --locked --workspace --exclude app --all-targets` failed due to missing system packages (e.g. `gdk-3.0.pc`) in the runner environment, however the focused module `server_integrations_omnichannel` passes all format, clippy, and unit tests independently.

issue_priority: "P1"
issue_category: "Backend"
issue_type: "Feature"
issue_label: "ohc:lane:messaging"
assignees: []
