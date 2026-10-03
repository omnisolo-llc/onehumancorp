outcome: blocked
issue_title: Implement Calendly Integration Build Fixes and Architecture Design
issue_description: >
  The instruction requests implementing Calendly webhook integration, normalizing
  events into Booking entities, exposing via gRPC, and building UI components (Flutter).
  However, the current authorized scope from RESEARCH.md explicitely states:
  "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines
  and harness adapters require evidence and the expansion gate in RESEARCH.md."
  Also, "The maintained UI for One Human Corp is Next.js/Tauri; creating additional
  frontends (e.g., Flutter) or rewrites is explicitly forbidden."

  The capability audit in docs/research/business_capability_and_usage_economics_audit.md
  states "Initial connector hypothesis is Google Workspace plus Stripe, subject
  to code/API/owner validation." and "Keep proposal, launch, retail and field-service
  paths as candidates until code verification and owner evidence justify selection."
  There is no evidence or authorization for Calendly integration in the active
  capability map. The research findings say: "Do not build another generic provider
  abstraction first. Extend the existing paths with reliable accounting...".

  Furthermore, the issue instructions demand Flutter UI creation for mobile which
  is explicitly forbidden by the operating contract. Therefore this issue is blocked.
  Evidence reviewed: RESEARCH.md, docs/development/native-build.md,
  docs/research/business_capability_and_usage_economics_audit.md,
  docs/research/native_migration_and_remediation.md.
  Skill provenance: .agent-scratch/superpowers/skills/using-superpowers/SKILL.md
  from upstream commit 8ca22dba9a94f28898bbce59f2537ff4d87c747d.
