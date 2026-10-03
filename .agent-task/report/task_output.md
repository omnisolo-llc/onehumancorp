outcome: blocked
issue_title: Native Rust Omnichannel Chat System & Chatwoot Retirement
issue_description: |
  **Evidence Context**
  The issue requests building a "Native Rust Omnichannel Chat System" to replace Chatwoot, including DB schema with RLS, Rust models, basic CRUD endpoints in `src/server/api`, and multi-tenant isolation, explicitly stating that it's a P0 and mentioning "Maya the baker or Carlos the handyman".

  **Verification**
  According to `RESEARCH.md` (Revision: 2026-09-18-usage-audit), "New epics need an explicit evidence-backed decision... A proposed change needs a current-code inventory, observed owner need and reuse analysis."

  Furthermore, the prompt states: "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require explicit evidence and authorization via the expansion gate in RESEARCH.md; otherwise, return a no_work or blocked finding."

  The `RESEARCH.md` also explicitly directs us to:
  1. "Correctness before expansion: recheck F01-F15 against the current code and remediation evidence. Prioritize unresolved false payment/completion claims, usage feedback/double accounting, tenant leakage, authority and budget defects."
  2. "Expand from evidence: add vertical-specific fulfillment, channels, HR/payroll, MRP or additional harnesses only when a current issue demonstrates why the reusable loop cannot satisfy the observed need."

  The requested new omnichannel chat system falls under new channels/adapters and is a major expansion. It is not currently authorized in the `RESEARCH.md` evidence gates, nor is it part of the F01-F15 remediation ledger which takes priority.

  I have also read `docs/research/business_capability_and_usage_economics_audit.md` which lists the real gaps and priorities. The issue proposes a completely new omnichannel backend (`inboxes`, `channels`, `contacts`, `conversations`, `messages`).

  This request requires explicit authorization via the expansion gate in `RESEARCH.md` before implementation can proceed.

  **Skills Provenance**
  - using-superpowers/SKILL.md (Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
