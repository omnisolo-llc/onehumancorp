outcome: blocked
issue_title: "Implement High-Performance Rust Omnichannel Gateway for Core Work Triage"
issue_description: |
  After fully reading `.agent-scratch/superpowers/skills/using-superpowers/SKILL.md` (and upstream `.agent-scratch/superpowers/README.md` at commit 8ca22dba9a94f28898bbce59f2537ff4d87c747d) and applying the superpowers workflow, I investigated the codebase to evaluate implementing an Omnichannel Gateway for Work Triage.

  According to `docs/research/business_capability_and_usage_economics_audit.md` (revision 2026-09-18-usage-audit):
  - "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md."
  - "The earlier $99 subscription, $299 setup, 300-step allowance, fixed cohort and margin targets are SUSPENDED HYPOTHESES, including wherever role prompts below repeat them."
  - "Do not implement them as billing or segment constraints."
  - "Keep proposal, launch, retail and field-service paths as candidates until code verification and owner evidence justify selection."
  - "Evaluation of managed API vs API-key/cloud-account billing is currently blocked pending real usage data and owner interviews."
  - "Evaluation of provider-permitted native-client subscription vs local inference is blocked due to missing specific provider access prerequisites."

  Furthermore, `docs/research/native_migration_and_remediation.md` states:
  - F15: "Existing commerce/fulfillment assets and varying owner stories contradict premature exclusive segment. Preserve modules; keep reusable workflow/owner evidence and commercial decisions separate from engineering readiness" (Status: Closed)
  - F11: "Named full-journey tests only delegate to a smoke helper" -> "Preserve smoke coverage; add actual mutation/state/provider-boundary acceptance tests without live credentials" (Status: Blocked (Verified no-work outcome))
  - F14: "No measured representative serving costs or owner outcomes" -> "Workload/cost instrumentation and repeatable benchmark/export; do not claim interviews, customer acceptance, real costs or competitive advantage without evidence" (Status: Blocked)

  The assigned issue requests implementing a new foundational architecture (Rust Omnichannel Gateway) to ingest WhatsApp/Instagram messages for AI routing. However, this constitutes an expansion into new channels (social media messaging/WhatsApp/Instagram) which lacks explicit authorization and the required expansion gate in `RESEARCH.md`. Specifically:
  - There is no documented owner interview or usage evidence validating this channel expansion.
  - The audit explicitly instructs: "Do not build another generic assistant, duplicate subsystem or broad ERP on the basis of this research."
  - The audit states: "A missing SDK, provider sandbox, signing credential or owner interview is a specific outstanding verification dependency, not permission to report success."
  - Expanding into WhatsApp/Instagram webhooks requires actual provider sandbox testing and evidence, which is currently unavailable.

  Therefore, I am returning a blocked finding as this implementation requires evidence and explicit authorization via an expansion gate in `RESEARCH.md` that is not currently present.
