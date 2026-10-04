outcome: blocked
issue_title: "Implement Multi-Channel POS Sync and Row-Level Inventory Reservation Architecture"
issue_description: |
  The requested feature ("Implement Multi-Channel POS Sync and Row-Level Inventory Reservation Architecture" / #34540) is blocked and out of scope under the current operating contract (Revision 2026-09-18-usage-audit).

  The issue describes implementing a Redis Redlock-based inventory reservation system combined with a PostgreSQL central ledger and Row-Level Security for multi-channel POS synchronization.

  However, the current `RESEARCH.md` and `AGENTS.md` explicitly state:
  - "New epics need an explicit evidence-backed decision; assigned concrete defect work may continue."
  - "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require explicit evidence and authorization via the expansion gate in RESEARCH.md."
  - "Defer new viral generators, referral badges, paywalls, agent marketplaces, additional harness adapters, visual workflow builders, simultaneous HR/payroll/MRP coverage, generic UI restyling and unsupported global compliance claims unless an accepted issue shows they block the selected business outcome."
  - "Gated expansion: Additional channels, industry packs, accounting integrations, physical operations, HR/payroll, POS, manufacturing, marketplaces and more harnesses: Require retained-customer need, measurable value and explicit strategy approval; existing capabilities are preserved."

  Since POS and multi-channel inventory synchronization engines fall under "POS" and "MRP/manufacturing" which require explicit authorization via the expansion gate in `RESEARCH.md`, and no such authorization exists for this new feature in the current scope, this feature request is blocked.

  **Skill Provenance:**
  - Loaded skills: `using-superpowers/SKILL.md`
  - Revision: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
