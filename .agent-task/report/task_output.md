issue_title: "Autonomous Local Delivery & Dispatch Mesh"
issue_description: |
  # Research Report: Autonomous Local Delivery & Dispatch Mesh

  ## Problem Statement
  Small business owners like Maya (the baker taking custom orders via Instagram) and Fatima (the food cart operator taking pre-orders) struggle with offering local delivery. They either lose 30% to major delivery apps (UberEats, DoorDash) or struggle manually dispatching local couriers or their own runners. They have to juggle multiple apps, manually enter customer addresses, and manually calculate delivery fees based on distance or zones. They need a zero-config way to offer local delivery where the AI handles dispatching delivery networks (like Uber Direct or DoorDash Drive) or local runners invisibly, automatically calculating fees and providing tracking updates to the customer, all from their phone.

  ## Findings
  - **Shopify Local Delivery:** Requires the merchant to manually create delivery zones, define prices, and use a separate app to route and dispatch deliveries. It is highly manual and requires the merchant to act as the dispatcher.
  - **Square On-Demand Delivery:** Requires desktop-first complex setup to integrate with DoorDash/Uber. It forces merchants to navigate clunky web interfaces and manage delivery rules manually.
  - **OneHumanCorp (OHC) Differentiation - "Invisible Dispatch":** Instead of making the merchant act as a dispatcher, OHC deploys an **Operations Agent**. When an order comes in for local delivery, the agent autonomously quotes the delivery via APIs (Uber Direct/DoorDash Drive), adds the transparent fee to the customer's checkout, and dispatches the courier when the order is marked "Ready" by the merchant.

  ## Proposed Next Steps
  We have detailed a high-level architectural design and implementation prompt in `docs/research/[architecture]_autonomous_local_delivery_dispatch_mesh.md`.

  The Implementer swarm should build the underlying architecture and UI, adhering to the mobile-first UX (375px baseline), Zero-Trust Isolation (SPIFFE), and AI integration guidelines without exposing any technical configuration to the merchant.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
