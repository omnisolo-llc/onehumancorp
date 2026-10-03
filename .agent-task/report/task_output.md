outcome: no_work
issue_title: "Chatwoot Retirement: Implement Native Rust Omnichannel Chat Inbox System"
issue_description: |
  The codebase already contains a Native Rust Omnichannel Chat Inbox System.
  The core domain models for `inbox`, `conversation`, `message`, and `contact`
  exist in `src/server/integrations/omnichannel/src/models.rs` and `src/server/domain/chat/mod.rs`
  with tenant_id isolation.
  Skill Provenance:
  - Loaded `using-superpowers` from `.agent-scratch/superpowers/skills/using-superpowers/SKILL.md`
  - Checked out https://github.com/obra/superpowers/ commit 8ca22dba9a94f28898bbce59f2537ff4d87c747d
