issue_title: Implement Offline-First Field Service Dispatch & AI Quoting Architecture
issue_description: |
  Finding: **blocked / no_work**

  The user requested implementing an Offline-First Field Service Dispatch & AI Quoting Architecture (Issue #34845).
  However, this feature is explicitly forbidden by the operating contract because it creates a new vertical/segment
  without explicit authorization.

  Per the instructions: "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters
  require explicit evidence and authorization via the expansion gate in RESEARCH.md; otherwise, return a no_work or
  blocked finding."

  This issue is targeted at "Carlos the Handyman" and field services (dispatch, quoting, route notes, PowerSync offline DB,
  Flutter mobile app).

  First, Flutter mobile app development is strictly forbidden in the OHC Operating Contract:
  "The maintained UI is Next.js/Tauri. Creating additional frontends (e.g., Flutter) or full rewrites is explicitly forbidden."

  Second, this creates a field service workflow, which is a new vertical. The contract requires proving the digital-service
  segment first:
  "Start with solo web, design, and marketing service professionals selling repeatable projects or retainers... Prove one client-to-cash loop and a paid subscription before expanding across industries."

  And specifically regarding Field Services:
  "Adjacent vertical after the first loop is validated; physical work remains with the owner/provider."

  Therefore, this work cannot be completed.

  Loaded skills:
  - skills/using-superpowers/SKILL.md

  Skill provenance revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
outcome: blocked
