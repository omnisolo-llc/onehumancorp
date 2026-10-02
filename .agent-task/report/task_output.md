outcome: no_work
issue_title: Implement Native Rust Omnichannel Chat System
issue_description: |
  The requested feature is already implemented and heavily integrated in the codebase.
  Specifically, `omni_inbox_messages`, `inbox_messages` and related DB migrations exist and handle messaging natively via rust `axum` and `sqlx`.
  There is a full rust API route implemented (`src/server/api/inbox_api.rs`) along with services in `src/server/services/inbox/service.rs`.
  Furthermore, E2E tests are already built around this feature (`omni_inbox`, etc.). This feature has already been merged as part of previous workflows and is visible across the repo.

  Skill provenance: Loaded `superpowers/skills/using-superpowers/SKILL.md` at revision 207efb9b.

  Final evidence / Execution results:
  # Executed test commands
  - `git clone --depth 1 https://github.com/obra/superpowers.git .scratch/superpowers` (Passed)
  - `cat .scratch/superpowers/skills/using-superpowers/SKILL.md` (Truncated output)
  - `ls -la` (Truncated output)
  - `head -n 50 RESEARCH.md` (Truncated output)
  - `cat docs/research/business_capability_and_usage_economics_audit.md` (Truncated output)
  - `grep -ri "chat" src/` (Truncated output)
  - `grep -ri "Inbox" src/` (Truncated output)
  - `ls -la src/server/integrations/` (Truncated output)
  - `cat src/server/integrations/omnichannel/mod.rs` (Failed)
  - `ls -la src/server/integrations/omnichannel` (Passed)
  - `ls -la src/server/integrations/omnichannel/src` (Passed)
  - `cat src/server/integrations/omnichannel/src/lib.rs` (Truncated output)
  - `cat src/server/integrations/omnichannel/src/models.rs` (Truncated output)
  - `cat src/server/integrations/omnichannel/src/repository.rs` (Truncated output)
  - `cat src/server/integrations/omnichannel/src/router.rs` (Truncated output)
  - `ls -la src/e2e/omni_inbox*` (Passed)
  - `grep -ri "api/v1/inbox" src/` (Truncated output)
  - `cat src/server/api/inbox_api.rs` (Truncated output)
  - `ls -la src/server/services/inbox/` (Passed)
  - `cat src/server/services/inbox/service.rs` (Truncated output)
  - `grep -ri "#36251" src/ docs/` (Passed)
  - `grep -ri "inbox_messages" src/` (Truncated output)
  - `cat src/server/lib.rs | grep -A 20 -n "CREATE TABLE omni_inbox_messages"` (Truncated output)
  - `cat src/server/lib.rs | grep -A 20 -n "CREATE TABLE inbox_messages"` (Truncated output)
  - `grep -rn "Omnichannel Chat Engine" src/` (Passed)
  - `make test-backend` (Failed)
  - `git branch` (Passed)
  - `grep -ri "chat_inboxes" src/server/migrations` (Truncated output)
  - `git status` (Passed)
  - `git log -n 5` (Passed)
  - `git branch -a` (Truncated output)
  - `grep -ri "17720328426270204614" src/ docs/` (Passed)
  - `git fetch origin taskmaster-omnichannel-chat-blocked-36459-15533180603170104854` (Passed)
  - `git show FETCH_HEAD --stat` (Truncated output)
  - `git log FETCH_HEAD -n 1` (Passed)
  - `git checkout FETCH_HEAD` (Truncated output)
  - `git status` (Truncated output)
  - `git checkout main` (Passed)
  - `git branch | grep taskmaster-omnichannel-chat-blocked-36459-15533180603170104854` (Passed)
  - `git checkout -b taskmaster-omnichannel-chat-blocked-36459-15533180603170104854 origin/taskmaster-omnichannel-chat-blocked-36459-15533180603170104854` (Truncated output)
  - `git checkout jules-9558322082142516297-207efb9b` (Passed)
  - `cargo check --locked --workspace --exclude app --all-targets` (Truncated output)
  - `make lint` (Failed)
  - `rm -rf .scratch/superpowers` (Passed)
  - `ls -la .scratch/` (Passed)

  # Verified trace limitations
  - `cat .scratch/superpowers/skills/using-superpowers/SKILL.md` (Truncated output)
  - `ls -la` (Truncated output)
  - `head -n 50 RESEARCH.md` (Truncated output)
  - `cat docs/research/business_capability_and_usage_economics_audit.md` (Truncated output)
  - `grep -ri "chat" src/` (Truncated output)
  - `grep -ri "Inbox" src/` (Truncated output)
  - `ls -la src/server/integrations/` (Truncated output)
  - `cat src/server/integrations/omnichannel/src/lib.rs` (Truncated output)
  - `cat src/server/integrations/omnichannel/src/models.rs` (Truncated output)
  - `cat src/server/integrations/omnichannel/src/repository.rs` (Truncated output)
  - `cat src/server/integrations/omnichannel/src/router.rs` (Truncated output)
  - `grep -ri "api/v1/inbox" src/` (Truncated output)
  - `cat src/server/api/inbox_api.rs` (Truncated output)
  - `cat src/server/services/inbox/service.rs` (Truncated output)
  - `grep -ri "inbox_messages" src/` (Truncated output)
  - `cat src/server/lib.rs | grep -A 20 -n "CREATE TABLE omni_inbox_messages"` (Truncated output)
  - `cat src/server/lib.rs | grep -A 20 -n "CREATE TABLE inbox_messages"` (Truncated output)
  - `grep -ri "chat_inboxes" src/server/migrations` (Truncated output)
  - `git branch -a` (Truncated output)
  - `git show FETCH_HEAD --stat` (Truncated output)
  - `git checkout FETCH_HEAD` (Truncated output)
  - `git status` (Truncated output)
  - `git checkout -b taskmaster-omnichannel-chat-blocked-36459-15533180603170104854 origin/taskmaster-omnichannel-chat-blocked-36459-15533180603170104854` (Truncated output)
  - `cargo check --locked --workspace --exclude app --all-targets` (Truncated output)
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
