outcome: "blocked"
issue_title: "Implement Native Rust Omnichannel Chat to Replace External Chatwoot Dependency"
issue_description: |
  Loaded Superpowers skills from revision 8ca22dba9a94f28898bbce59f2537ff4d87c747d:
  - skills/using-superpowers/SKILL.md
  - skills/brainstorming/SKILL.md

  Verification checks and outcomes:
  - Checked `docs/research/business_capability_and_usage_economics_audit.md` and `RESEARCH.md`.
  - Confirmed via `RESEARCH.md` that "New verticals, channels, agent marketplaces... require evidence and the expansion gate in RESEARCH.md."
  - The implementation prompt requests adding Meta (IG/WhatsApp), Email, and Web Chat channels via a new Rust microservice.
  - This constitutes adding new channels which is explicitly blocked pending owner evidence and the expansion gate.
  - Furthermore, Chatwoot was already removed per `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md` and `deploy/tests/no_chatwoot_residue_test.sh`.

  Expected owner result: Avoid building unverified features without explicit customer evidence for the omnichannel use case.
issue_priority: "P0"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
