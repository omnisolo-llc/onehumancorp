outcome: blocked
issue_title: "Implement Intelligent Tap-to-Pay Visibility and Cash/External Logging for Physical Retail (Priya)"
issue_description: |
  The requested issue attempts to build a POS/physical retail payment logging UI and database schema (Tap-to-Pay, Cash, External Terminals) directly into the Next/Tauri application, adding new tables and expanding into a physical-retail channel.

  However, according to the "2026-09-18-usage-audit" revision of RESEARCH.md and the required execution guidelines:
  1. "New epics need an explicit evidence-backed decision; assigned concrete defect work may continue."
  2. "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md."
  3. "Existing commerce, fulfillment and service modules preserved; earlier exclusive agency segment and fixed-price targets remain suspended. Blocked due to missing prerequisites and owner economic/metric data." (from native_migration_and_remediation.md finding F15).
  4. The prompt says: "In the One Human Corp repository, requests to build native replacements for external services, or expand into new verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines, and harness adapters, require explicit authorization, evidence, and an expansion gate in RESEARCH.md; otherwise, return a no_work or blocked finding."

  This PR request clearly falls under "expand into new verticals, channels... POS engines". As there is no explicit expansion gate or evidence available in RESEARCH.md authorizing POS development at this time, this task is blocked.

  Loaded skills: `.agent-scratch/superpowers/skills/using-superpowers/SKILL.md`
  Upstream revision: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
