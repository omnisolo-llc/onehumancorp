issue_title: "Unified Multi-Channel Inventory Sync & POS Architecture"
issue_description: |
  ## Problem Statement
  Solopreneurs and small business operators like Priya (boutique owner) require seamless inventory tracking between their online storefronts (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking mechanism and a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases, leading to customer dissatisfaction and operational overhead.

  ## Research Report
  Our competitive analysis indicates that while platforms like Shopify provide extensive POS capabilities, they are often too complex for micro-SMEs and can suffer from disjointed inventory updates between online and in-store sales unless costly third-party integrations are used. Dedicated POS solutions like Square offer robust hardware but lack the integrated agentic workflow automation necessary to unify operations effortlessly. OHC's differentiator is to leverage AI agents to coordinate these operations invisibly, ensuring real-time consistency and automating follow-up actions like customer notifications and restock alerts.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer as Online Customer
      participant Priya as POS Client (Mobile)
      participant Gateway as API Gateway
      participant OperationsAgent as Operations Agent
      participant Redis as Redis Redlock
      participant DB as PostgreSQL (Central Ledger)

      Note over Priya, DB: In-Store Purchase Flow
      Priya->>Gateway: Initiate Checkout (Stripe Terminal)
      Gateway->>Redis: Acquire Lock `ohc:lock:{tenant_id}:inventory:{product_id}`
      Redis-->>Gateway: Lock Acquired (15s TTL)
      Gateway->>DB: Reserve Inventory (Row-level lock)
      DB-->>Gateway: Reservation Confirmed
      Gateway-->>Priya: Proceed to Payment

      Note over Customer, DB: Simultaneous Online Attempt
      Customer->>Gateway: Add to Cart (Same Item)
      Gateway->>Redis: Attempt Lock
      Redis-->>Gateway: Lock Failed (Reserved)
      Gateway-->>Customer: Item Unavailable / Reserved

      Note over Priya, DB: Completion & Agent Action
      Priya->>Gateway: Payment Success
      Gateway->>DB: Deduct Inventory & Release Lock
      DB-->>Gateway: Committed
      Gateway->>OperationsAgent: Event: Stock Level Updated
      OperationsAgent->>OperationsAgent: Check Low Stock Threshold
      OperationsAgent->>Priya: Notification: Suggest Restock Plan
  ```

  ### Mobile UX Flow
  1. The POS interface on the OHC mobile app operates flawlessly on a 375px viewport.
  2. Touch targets for inventory adjustment and checkout are large (≥ 44x44px).
  3. When an item is added to the POS cart, an optimistic UI update reflects the reservation, with rollback if the Redis lock fails.
  4. The Operations Agent provides actionable notifications (e.g., "Restock suggested for Red Dress") directly in the mobile feed.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors stock levels across channels, triggers low-stock alerts, and coordinates with sync mechanisms.
  - **Finance Agent:** Processes payment splits and correlates POS data with online purchases.
  - **Customer Success Agent:** Automatically updates online storefront availability and notifies customers of changes.

  ## Implementation Prompt
  **Feature:** OHC Unified Multi-Channel Inventory Sync & POS
  **Target Persona:** Priya the Boutique Owner
  **User-Facing Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item. The Operations Agent manages this invisibly and suggests restocks when necessary.
  **Critical User Journey (CUJ):**
  1. Priya logs into the OHC mobile app (POS mode) while an online customer browses her storefront.
  2. Priya processes an in-store sale using Stripe Terminal integration.
  3. The system applies a 15-second Redis Redlock to reserve the item.
  4. The online customer attempts to add the same item to their cart and is informed it is currently reserved/unavailable.
  5. Upon payment completion, the stock is permanently deducted, and Priya receives a restock suggestion from the Operations Agent if the threshold is met.

  **Priority:** P1
  **Estimated Scope:** Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
