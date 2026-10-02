outcome: blocked
issue_title: "Implement Native Omnichannel Unified Inbox & Agentic Customer Triage"
issue_description: |
  The issue requests implementing a native Rust-based omnichannel chat engine with new channels (WhatsApp, Instagram DMs, Email, SMS) and an AI Triage Agent.
  However, `RESEARCH.md` states: "Evaluate OHC-11/12 or an additional connector/channel" only "After gates" when there is "A retained customer's observed need, willingness-to-pay evidence, reuse/integration comparison and explicit expansion decision".
  The current audit ledger (`docs/research/business_capability_and_usage_economics_audit.md`) shows OHC-09 is blocked due to missing retention data and actual owner economics. The expansion gate has not been met.

  Superpowers workflow provenance:
  Loaded skill `using-superpowers` from `/tmp/superpowers/skills/using-superpowers/SKILL.md` (revision `8ca22dba9a94f28898bbce59f2537ff4d87c747d`).
  Loaded skill `brainstorming` and `writing-plans` as part of the exploration.
  The workflow rules state: "New verticals, channels, agent marketplaces... require evidence and the expansion gate in RESEARCH.md."

  Verification blockers:
  During `make test && make lint`, the test suite failed on `make build-web` with the error `sh: 1: next: not found` inside `scripts/build-web.mjs`.
  This is a local environment/dependency issue affecting the un-modified `main` branch codebase. No source files were modified.
