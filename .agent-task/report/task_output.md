issue_title: "Implement Autonomous Shipping & Hyperlocal Fulfillment Mesh"
issue_description: |
  # Autonomous Shipping & Hyperlocal Fulfillment Mesh

  ## Problem Statement
  Small business owners face friction manually routing orders between national shipping (USPS) and local delivery (Uber Direct). This requires manual configuration, rate comparison, and distinct workflows.

  ## Research Report
  - **Shopify**: Relies on third-party apps for local delivery routing.
  - **Wix/Squarespace**: "Manual local delivery" where merchants fulfill themselves.
  - **Opportunity**: OHC can provide zero-touch fulfillment, automatically routing based on distance, dimensions, and cost to completely abstract logistics.

  ## Architecture & Design
  The system uses an AI routing agent to determine the optimal fulfillment path, utilizing a unified `FulfillmentJob` ledger. The UI provides a mobile-first, zero-touch experience on a 375px viewport with a single unified status, eliminating configuration panels for the merchant.

  Please see `docs/research/[architecture]_autonomous_shipping_and_hyperlocal_fulfillment_mesh.md` for full implementation details, Mermaid diagrams, and acceptance criteria.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []