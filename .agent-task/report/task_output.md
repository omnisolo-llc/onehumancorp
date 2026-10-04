outcome: blocked
issue_title: "AI Unified Communications & Multi-Channel Work Triage Architecture"
issue_description: |
  **Evidence & Audit Gap:** The issue requests implementing a multi-channel unified communications triage architecture (including Instagram DMs, WhatsApp, SMS), powered by a new "The Ambassador" AI agent, and a unified feed. However, according to the `docs/research/business_capability_and_usage_economics_audit.md` and the final OHC scope check:
  - "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md."
  - "Do not implement them as billing or segment constraints."
  - "No eligible task or already-satisfied criteria means an evidence-backed no-work result. Do not manufacture refactors, viral tools, new harness integrations or changed files to justify a session."

  Currently, there is no evidence or authorization for new channel integrations (like Instagram or WhatsApp APIs), nor for the "The Ambassador" agent framework described. Furthermore, the repository's foundational build paths are failing (e.g., `make lint` and `make test` fail out of the box because the `next` module is missing during the `make build-web` step: `sh: 1: next: not found`), violating the required acceptance gates for any actual implementation task. Therefore, this issue must be returned as a blocked no-work finding pending authorization, evidence of need, and a fixed build environment.
