outcome: blocked
issue_title: "AI Unified Inbox & Omnichannel Customer Memory"
issue_description: |
  The requested task is "AI Unified Inbox & Omnichannel Customer Memory".

  According to the explicit instructions in `AGENTS.md` and `RESEARCH.md` (Revision: 2026-09-18-usage-audit), we must rely on the explicit expansion gate for new features like "Omnichannel". Specifically, `RESEARCH.md` states:
  > Expand from evidence: add vertical-specific fulfillment, channels, HR/payroll, MRP or additional harnesses only when a current issue demonstrates why the reusable loop cannot satisfy the observed need. No feature quota, random framework upgrade or forced cosmetic diff.

  And `AGENTS.md` reinforces:
  > New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md.

  There is currently no existing feature implementation of "AI Unified Inbox & Omnichannel Customer Memory" beyond `.mock-contract.ts` test files and some stub webhook endpoints. This represents a new feature ("new channels", "AI Unified Inbox") that requires explicit authorization and evidence, which has not been provided. The audit documents do not record this as a remediated finding that we just need to test, but rather a new product area without an approved expansion gate.

  Therefore, the implementation of this feature is blocked.
  Skill loaded: skills/using-superpowers/SKILL.md, Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
