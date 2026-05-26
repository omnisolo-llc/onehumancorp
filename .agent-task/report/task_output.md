issue_title: "Implement Autonomous Capital & Cashflow Engine"
issue_description: |
  We have researched the market gap for micro-SMB working capital and designed an architecture for the Autonomous Capital & Cashflow Engine.

  The full design document can be found at `docs/research/[architecture]_autonomous_capital_and_cashflow_engine.md`.

  The engine uses ledger data to proactively trigger capital advance offers and automates repayment by directly deducting a percentage of daily sales via OHC. This capability aims to rival Square Loans and Shopify Capital by leveraging our 360-degree view of business health (inventory, bookings, POS).

  Next steps: The implementer swarm should use the implementation prompt in the design document to begin work on the database schema, multi-tenant API endpoints, and mobile-first frontend flow.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []