issue_title: "OHC Unified Multi-Channel Inventory Sync & POS"
issue_description: |
  **Outcome: blocked**

  The feature brief for "OHC Unified Multi-Channel Inventory Sync & POS" (Issue #34450) calls for:
  1. Redis Redlock inventory reservation service integrated into the checkout flow.
  2. Refining the `TerminalSession` schema for offline-sync reconciliation.
  3. Extending the Operations Agent.

  However, following the project's evidence and architecture rules in `docs/research/business_capability_and_usage_economics_audit.md` (Revision: 2026-09-18-usage-audit), and `RESEARCH.md` (Revision: 2026-09-18-usage-audit):

  - "New epics need an explicit evidence-backed decision".
  - "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require explicit evidence and the expansion gate in RESEARCH.md."
  - "Preserve existing commerce, fulfillment and service modules... Keep reusable workflow/owner evidence and commercial decisions separate from engineering readiness."
  - The feature involves introducing new dependencies (Redis Redlock, which is a new architectural component for caching/locks) and expanding into a POS engine, which violates the expansion gate rule in RESEARCH.md as the prerequisite real-world evidence and strategy approvals have not yet been satisfied.
  - The audit explicitly notes that existing workflows (like `TerminalSession` and `Operations Agent` mocks found via grep) are not fully realized production capabilities and should not be expanded without resolving the underlying business metric evaluation ("Blocked due to missing prerequisites and owner economic/metric data").
  - Therefore, implementing a robust distributed POS sync protocol with Redis Redlock is explicitly **blocked** until the core billing, usage economics, and single-channel pilot are fully completed and measured.

  Loaded skills:
  - `skills/using-superpowers/SKILL.md`
  - `skills/systematic-debugging/SKILL.md`
  - `skills/systematic-debugging/root-cause-tracing.md`

  Revision hash for superpowers workflow: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
outcome: "blocked"
