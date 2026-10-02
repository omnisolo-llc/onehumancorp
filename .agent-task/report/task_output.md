outcome: no_work
issue_title: "Native Rust Omnichannel Chat System Implementation"
issue_description: |
  Issue #36248 requests the implementation of a native Rust Omnichannel Chat System to replace Chatwoot.
  Verification of the codebase shows this is already implemented and Chatwoot has been fully removed.

  # Verified evidence
  - Loaded Superpowers skills: `skills/using-superpowers/SKILL.md` and `skills/brainstorming/SKILL.md` from revision 8ca22dba9a94f28898bbce59f2537ff4d87c747d.
  - `docs/reports/production_agent_optimization_report.md` documents "CHAT-00 — Chatwoot removal" as completed on 2026-07-13.
  - `src/server/migrations/1009_native_omnichannel_chat.sql` already contains the requested PostgreSQL schema (`chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, `chat_messages`) with strict `tenant_id` RLS.
  - Source search for `chatwoot` using `grep -ri "chatwoot" src/` returns zero results, confirming Chatwoot code is completely removed.

  # Executed test commands
  ```
  ls -la
  cat RESEARCH.md | head -n 40
  cat docs/research/native_migration_and_remediation.md | head -n 40
  cat docs/research/native_migration_and_remediation.md | tail -n 40
  grep -ri "chatwoot" . | grep -v "\.git"
  cat docs/reports/production_agent_optimization_report.md | head -n 40
  cat docs/reports/production_agent_optimization_report.md | tail -n 40
  grep -ri "inbox" src/ server/
  grep -ri "chatwoot" src/
  ls -la src/server/migrations/*omni*
  cat src/server/migrations/1009_native_omnichannel_chat.sql
  mkdir -p .scratch/superpowers && cd .scratch/superpowers && git clone --depth 1 https://github.com/obra/superpowers.git .
  cd .scratch/superpowers && git rev-parse HEAD
  ```

  Outcome is no-work since the requested functionality is already fully present in the codebase.
