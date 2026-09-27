issue_title: "Economics and owner outcomes data blocked"
issue_description: |
  **Research findings & outcomes:**

  This report documents the status of F14: "No measured representative serving costs or owner outcomes" as detailed in `docs/research/native_migration_and_remediation.md` and `docs/research/business_capability_and_usage_economics_audit.md`.

  **Problem Statement:**
  The project lacks measured representative serving costs and validated owner economic outcomes. We cannot confidently deploy cost-bounded client work or compute/API charging frameworks without concrete data detailing owner usage limits, provider costs, margins, and willingness to pay.

  **Research Report:**
  As requested by the 2026-09-18-usage-audit, a review of existing data around workload usage and business capability was performed. The earlier assumptions ($99 subscription, 300-step allowance, exclusive design segment, etc.) are suspended hypotheses and not reliable for building a new billing architecture. The task explicitly requires "measured representative serving costs or owner outcomes", which currently do not exist in the repository or available data context. Therefore, implementing billing rules, resource usage models, or specific feature segments based on suspended hypotheses would violate the strict evidence gates.

  **Design Doc:**
  ""

  **Implementation Prompt:**
  ""

  **Priority:**
  High (Blocker)

  **Estimated Scope:**
  Unknown (requires external owner feedback and measurement infrastructure first)

  **Superpowers skill provenance:**
  Loaded skills: using-superpowers, brainstorming.
  Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  Verified limitations: Missing owner outcomes data and workload usage metrics.
  Executed test commands, checks, and outcomes as final evidence: `make test-node` (passed once missing dependencies were resolved), `npm run test:desktop-ui` (passed).
issue_priority: "P0"
issue_category: "economics"
issue_type: "blocked no-work finding"
issue_label: "F14"
assignees: []
