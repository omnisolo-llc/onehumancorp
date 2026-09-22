issue_title: "✍️ Scribe: Document how an owner accomplishes verified business work"
issue_description: |
  **Title**: Scribe: Document how an owner accomplishes verified business work

  **Problem Statement**:
  Document how an owner accomplishes verified business work with the current product. Explain setup, connected accounts, standing authority, evidence, cost, exceptions and recovery in plain language.

  **Research Report**:
  - **Setup**: New owners configure their store by accessing the Dashboard, establishing basic business settings, and enabling billing options.
  - **Connected Accounts**: The platform supports connecting Stripe for payments. Verified tenant OpenAI keys bind to the provider origin, supporting Bring Your Own Key (BYOK) setups.
  - **Standing Authority**: The AI agents operate within bounded authority defined by the tenant's policies. Agent messages serve as coordination, not proof of external success. When actions exceed granted authority, agents halt and request explicit approval.
  - **Evidence**: Verified business actions are captured in the telemetry and usage databases. Provider receipts, invoice balances, and durable work records form the foundation of truth. Final evidence explicitly names the loaded skills and exact checks executed.
  - **Cost**: Inference costs, reserved compute, and platform fees are tracked. The system ensures customer-direct inference is not charged twice. Hard budget caps restrict unapproved spending.
  - **Exceptions and Recovery**: Errors during provider communication trigger bounded recovery actions. If an action fails due to missing credentials, exhausted budgets, or declined payments, the workflow persists in an unresolved state for owner review without dropping the business context.

  **Scope**: Research and documentation analysis.
  **Dates**: 2026-09-18
  **Metric Definitions**: Verified business work is defined by persisted durable records bound to tenant/task/attempt.
  **Uncertainties**: Replay-safe settlement for customer payment collection remains incomplete on the new usage API.
  **Study Populations**: None, research was conducted analytically over the current codebase and documentation.
  **Skill Provenance**: Used superpowers:using-superpowers, superpowers:writing-plans, superpowers:brainstorming, superpowers:executing-plans, superpowers:verification-before-completion, and superpowers:requesting-code-review. Loaded from revision 5bf4e78011075bcfc0dc295f0724994cd123ee71.

issue_priority: "P1"
issue_category: "Documentation"
issue_type: "Research"
issue_label: "Docs"
assignees: []
