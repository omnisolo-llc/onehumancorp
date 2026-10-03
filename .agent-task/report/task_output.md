outcome: no_work
issue_title: "[Architecture] Native Rust Omnichannel Inbox & Chatwoot Replacement"
issue_description: |
  **Evidence & Skill Provenance**
  - Loaded Skill: `superpowers:using-superpowers`
  - Loaded Paths: `.agent-scratch/superpowers/skills/using-superpowers/SKILL.md`
  - Upstream Revision: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`

  **Finding**
  The issue requests building a "Native Rust Omnichannel Inbox & Chatwoot Replacement" (a new vertical/channel expansion). However, according to the strict guidelines in `RESEARCH.md` (Revision: 2026-09-18-usage-audit), "New verticals, channels, agent marketplaces... require evidence and the expansion gate in RESEARCH.md."

  The current strategy explicitly mandates: "Correctness before expansion: recheck F01-F15 against the current code... Expand from evidence: add vertical-specific fulfillment, channels... only when a current issue demonstrates why the reusable loop cannot satisfy the observed need."

  Because there is no explicit authorization or expansion gate in `RESEARCH.md` for this omnichannel inbox replacement, this task is blocked and no implementation work is permitted.
