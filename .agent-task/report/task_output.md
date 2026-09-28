issue_title: "Compute/API charging and BYOK Evaluation"
issue_description: |
  # Compute/API charging and BYOK Evaluation

  ## Skill Provenance
  - Loaded superpowers:using-superpowers from /tmp/superpowers/skills/using-superpowers/SKILL.md (revision 528f04eab).

  ## Scope
  Evaluate compute/API charging and customer BYOK (Bring Your Own Key) access based on current source evidence.

  ## Uncertainties & Findings
  1. The current implementation lacks measured deployment cost and reconciled provider invoices for accurate billing.
  2. Provider native client subscriptions (e.g. ChatGPT Plus) cannot simply be pooled or used for generic API access.
  3. API balances, subscription quota, and customer cash are not interchangeable and require tenant-specific read/write boundaries.
  4. A true billing meter needs to capture durable, deduplicated events with tenant, payer, and provider request IDs.
issue_priority: "P1"
issue_category: "research"
issue_type: "evaluation"
issue_label: "ohc:lane:finance"
assignees: ["Bolt"]
