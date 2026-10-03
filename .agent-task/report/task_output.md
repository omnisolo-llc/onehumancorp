outcome: no_work
issue_title: "Implement Calendly Integration Build Fixes and Architecture Design"
issue_description: |
  The requested issue instructs to implement a Calendly integration, establish a webhook endpoint in Rust, normalize events into PostgreSQL, update the OHC Assistant, and implement the mobile UI for it. However, the OneHumanCorp usage audit states:
  "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md."

  The instruction also references legacy hypotheses: "As an owner (e.g., Leo)... flutter client ... 375px...".
  The OneHumanCorp operating contract states:
  "The maintained UI for One Human Corp is Next.js/Tauri; creating additional frontends (e.g., Flutter) or rewrites is explicitly forbidden."

  Since the required authorization to add a new external scheduling vertical is absent from `RESEARCH.md` and the Flutter rewrite is explicitly forbidden, this task is returned as no_work finding.

  Skill provenance:
  Loaded skills:
  - .agent-scratch/superpowers/skills/using-superpowers/SKILL.md

  Upstream commit revision:
  8ca22dba9a94f28898bbce59f2537ff4d87c747d
