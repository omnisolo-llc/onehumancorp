issue_title: "Implement Universal Subscription Engine"
issue_description: |
  **Research Report**:
  OHC lacks a unified data model and user experience for recurring revenue across physical products, digital goods, and services. A new Universal Subscription Engine is needed to abstract Stripe Billing complexity and integrate with our omnichannel ledger and scheduling system.

  **Proposed Next Steps**:
  - Backend must build out the subscription data models utilizing an entitlement-based system to support all business variations, mapped via Stripe Billing.
  - Implement Zero-Trust multi-tenancy.
  - Frontend to build a mobile-first (375px) subscription builder form based on the Glassmorphism OHC design token.
  - AI Agents (Customer Success, Operations, and Finance) must integrate with the new database models to automate tasks such as dunning, booking, and reconciliations.

  Full details in `docs/research/[architecture]_universal_subscription_engine.md`.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []