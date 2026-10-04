issue_title: "AI Unified Inbox & Omnichannel Customer Memory"
issue_description: |
  The task requests implementing an 'AI Unified Inbox & Omnichannel Customer Memory' feature integrating Instagram DMs, WhatsApp, SMS, and email. The request cites an 'Omnichannel Gateway' and 'The Ambassador' agent to draft contextual replies for small business owners.

  However, reviewing the canonical `RESEARCH.md` and current code state shows this is unauthorized:
  1. The `RESEARCH.md` document (Revision: 2026-09-18-usage-audit) explicitly states: "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md."
  2. Further, it mandates: "Expand from evidence: add vertical-specific fulfillment, channels... only when a current issue demonstrates why the reusable loop cannot satisfy the observed need."
  3. The current capabilities list in `RESEARCH.md` only mentions "One email/calendar stack and one payment processor; existing document/artifact tools for one supported deliverable. Initial hypothesis: Google Workspace + Stripe."
  4. The codebase does not have an "Omnichannel Gateway". While "The Ambassador" string is found in some onboarding/customer success areas, there's no authorized Omnichannel integration pathway documented.
  5. The prompt itself overrides older instructions: "The earlier $99 subscription, $299 setup, 300-step allowance, fixed cohort and margin targets are SUSPENDED HYPOTHESES, including wherever role prompts below repeat them. Do not implement them as billing or segment constraints." and "New epics need an explicit evidence-backed decision; assigned concrete defect work may continue."

  Since this is a massive new epic involving new channels (Instagram, WhatsApp, SMS) without explicit authorization in `RESEARCH.md` through the expansion gate, it is a blocked request based on the project's strict evidence-based constraints.

  I am returning a blocked outcome as the requested features are out of the currently authorized scope in RESEARCH.md.

  Skills provenance:
  - Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - Loaded skills:
    - skills/using-superpowers/SKILL.md
    - skills/brainstorming/SKILL.md
    - skills/writing-plans/SKILL.md
    - skills/systematic-debugging/SKILL.md
    - skills/subagent-driven-development/SKILL.md
    - skills/executing-plans/SKILL.md
    - skills/verification-before-completion/SKILL.md
issue_priority: P2
issue_category: integrations
issue_type: feature
issue_label: [blocked, ohc:lane:integrations]
assignees: []
outcome: blocked
