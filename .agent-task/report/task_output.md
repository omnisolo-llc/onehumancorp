outcome: no_work
issue_title: "Implement Native Rust Omnichannel Chat to Replace External Chatwoot Dependency"
issue_priority: "P0"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
issue_description: |
  **No-work finding: Chatwoot has already been removed and the Native Omnichannel Inbox is actively in place.**

  The assigned task (#36907) requested replacing the external Chatwoot dependency with a native Rust omnichannel chat system. Extensive evidence throughout the repository confirms this migration has already been completed:

  1. **Historical Documentation & Evidence:**
     - The file `docs/reports/production_agent_optimization_report.md` includes a detailed section `### CHAT-00 — Chatwoot removal`. It states: "**Status (2026-07-13): Removed from the active application and deployment graph.** The repository owner confirmed in this thread on 2026-07-13 that there was no real-customer or production Chatwoot deployment or data... The native OmniSolo omnichannel inbox remains in place".
     - `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md` outlines the entire design for the replacement native omnichannel chat, showing it was designed and rolled out previously.
     - `docs/superpowers/plans/2026-07-13-chatwoot-removal.md` contains the executed plan to remove Chatwoot.
  2. **Codebase Footprint:**
     - A strict residue guard test script `deploy/tests/no_chatwoot_residue_test.sh` is present and runs in CI (`.github/workflows/ci.yml`) to ensure no Chatwoot artifacts are reintroduced.
     - A search across the codebase for `chatwoot` (ignoring historical docs and guards) returns no results in the active application or deployment graph (`src/`, `deploy/helm/`, `deploy/docker-compose.yml`, etc).

  Per the strict task operating contract (revision 2026-09-18-usage-audit), since the assigned work (the removal of Chatwoot and introduction of the native alternative) is already complete, this must be reported as a justified `no_work` outcome rather than generating a duplicate subsystem or making dummy changes.

  Loaded Superpowers skills/revision: using-superpowers (8ca22dba9a94f28898bbce59f2537ff4d87c747d).
  Checks executed: `git grep -i chatwoot`, file reads on `docs/reports/production_agent_optimization_report.md` and `docs/research/native_migration_and_remediation.md`.
  Outcomes: Verified that Chatwoot is completely removed and the native Rust omnichannel inbox functionality has been implemented.
