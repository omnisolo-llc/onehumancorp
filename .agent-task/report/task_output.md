issue_title: "[Architecture] Implement OHC Unified Multi-Channel Inventory Sync & POS"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners with multi-channel operations (online + in-store), such as Priya the boutique owner, struggle with disjointed inventory management. Current platforms like Shopify fail micro-SMEs due to complexity, while Square/Stripe lack integrated agentic workflow automation. Without a robust distributed sync protocol, merchants face double-booking and out-of-stock scenarios during simultaneous online and offline purchases. The non-technical owner needs an invisible, agent-coordinated system that prevents these conflicts.

  ## Research Report
  - **Market Context**: Competitors require costly third-party integrations or complex setups to unify in-store (tap-to-pay) and online inventory.
  - **OHC Gap**: OHC currently lacks a real-time, strongly consistent inventory locking and caching mechanism.
  - **Findings**: Implementing a central ledger with distributed locks (Redis Redlock) and an offline-first POS client with eventual consistency will close this gap.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer/POS Checkout] --> B{Distributed Lock - Redis Redlock}
      B -->|Acquire Lock| C[Process Transaction]
      B -->|Lock Failed| D[Show Graceful Out of Stock Message]
      C --> E[Update Central Ledger - PostgreSQL]
      C --> F[Async Reconcile & Offline Sync]
      E --> G[Operations Agent: Monitor Stock]
      G -->|Low Stock| H[Notify Owner to Restock]
  ```

  ### Mobile UX Flow
  - **Mobile POS Client (375px viewport)**: Optimistic UI updates for inventory changes during checkout.
  - **Checkout Flow**:
    1. Cashier (or online customer) initiates purchase.
    2. System seamlessly applies a 15-second Redis Redlock (`ohc:lock:{tenant_id}:inventory:{product_id}`).
    3. If concurrent purchase is attempted on the other channel, the secondary buyer receives a friendly "Item just sold out" notification.
  - **Touch Targets**: All inventory adjustment buttons must be ≥ 44x44px.

  ### AI Agent Integration
  - **Operations Agent ("The Manager")**: Actively monitors central ledger stock levels. Handles sync conflicts, triggers low-stock alerts, and dynamically drafts restock orders.
  - **Customer Success Agent ("The Ambassador")**: Intercepts failed checkouts due to concurrency and suggests alternatives to the customer.
  - **Finance Agent ("The Accountant")**: Correlates split POS data with online purchases for unified reporting.

  ## Implementation Prompt
  Implement the Redis Redlock inventory reservation service and integrate it into the checkout and POS flow. Modify the `TerminalSession` data schema to handle offline-sync reconciliation with the PostgreSQL central ledger. Extend the Operations Agent to monitor real-time stock levels, resolve sync conflicts, and dispatch low-stock push notifications to the owner. Ensure the mobile POS interface provides optimistic UI updates and handles lock failures gracefully.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
