outcome: blocked
issue_title: OHC Unified Multi-Channel Inventory Sync & POS
issue_description: |
  The assigned task (Issue #34983: OHC Unified Multi-Channel Inventory Sync & POS) requests the implementation of a centralized inventory and distributed Point-of-Sale (POS) synchronization architecture, including Redis Redlock for inventory reservation and a PostgreSQL ledger.

  However, this task represents a major new feature epic (expanding into retail, POS engines, and multi-channel inventory sync) which is explicitly forbidden by the current operating contract (RESEARCH.md revision 2026-09-18-usage-audit) unless explicit authorization, evidence, and an expansion gate are met. The contract states: "New epics need an explicit evidence-backed decision; assigned concrete defect work may continue." and "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md."

  Furthermore, the required evidence (owner interviews, real usage data) for evaluating compute/API charging and BYOK is currently documented as blocked in `docs/research/business_capability_and_usage_economics_audit.md` (Implementation Blockers recorded 2026-09-19).

  Therefore, I am returning a blocked finding as the requested implementation lacks the required authorization and evidence in RESEARCH.md.

  Note: `make test` failed with a pre-existing issue related to Next.js Turbopack and TailwindCSS (It looks like you're trying to use tailwindcss directly as a PostCSS plugin...). I did not modify the Next.js setup as this is a blocked / no-work issue due to policy.

  Skill Provenance:
  - Loaded path: `.agent-scratch/superpowers/skills/using-superpowers/SKILL.md`
  - Upstream commit revision: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
