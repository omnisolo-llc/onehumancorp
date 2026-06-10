issue_title: "Implement Real-time Inventory Distributed Lock and Sync Protocol"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Priya (boutique owner) requires seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases. Legacy platforms like Shopify require third-party tools or high-tier plans to accomplish this.

  ## Research Report
  - **Market Mapping:** Competitors like Shopify dominate e-commerce with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools are employed. Square provides robust POS hardware but lacks the integrated, agentic workflow automation needed to unify operations.
  - **The OHC Differentiator:** By leveraging centralized PostgreSQL ledgers and Redis Redlock for distributed reservation locks, OHC can ensure that a physical sale immediately reserves the inventory, preventing double-bookings online. Furthermore, the Operations Agent can invisibly handle restock workflows and discrepancy alerts.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant App as POS Client (Priya)
      participant Store as Online Storefront
      participant API as OHC API
      participant Redis as Distributed Lock (Redlock)
      participant DB as PostgreSQL Ledger
      participant OpsAgent as Operations Agent

      App->>API: Initiate POS Checkout (Item X)
      API->>Redis: Acquire Lock `ohc:lock:{tenant}:inventory:{item}`
      Redis-->>API: Lock Granted
      Store->>API: Initiate Online Checkout (Item X)
      API->>Redis: Attempt Lock `ohc:lock:{tenant}:inventory:{item}`
      Redis-->>API: Lock Denied
      API-->>Store: Return Graceful "Out of Stock"
      App->>API: Complete POS Sale (Payment OK)
      API->>DB: Update Ledger (Deduct Item X)
      API->>Redis: Release Lock
      API->>OpsAgent: Broadcast `InventoryUpdated` Event
      OpsAgent->>App: Send "Item X is sold out. Draft restock?"
  ```

  ### Mobile UX Flow
  - Layouts must perfectly fit a 375px viewport.
  - Touch targets for POS checkout and inventory adjustment are ≥ 44x44px.
  - POS client must implement optimistic UI updates during the transaction, seamlessly gracefully rolling back if the transaction fails or the lock is lost.

  ### AI Agent Integration
  - **Operations Agent:** Monitors `InventoryUpdated` and `StockReserved` events to actively trigger low-stock alerts or coordinate with external systems.
  - **Customer Success Agent:** Can intercept failed online checkouts due to lock denial to automatically email the customer an alternative product recommendation or restock notification sign-up.

  ## Implementation Prompt
  **Target Persona:** Priya the Boutique Owner

  **Outcome:** Implement the centralized inventory sync and distributed POS locking mechanism so that an in-store transaction instantly locks inventory to prevent online double-booking.

  **Acceptance Criteria:**
  1. Add a Redis Redlock mechanism in the backend services to temporarily reserve inventory during checkout flows (both online and POS). The lock key pattern must follow `ohc:lock:{tenant_id}:inventory:{product_id}`.
  2. Implement optimistic concurrency in the central PostgreSQL ledger for final inventory deduction.
  3. Emit an `InventoryUpdated` event that the Operations Agent can consume.
  4. Build a POS endpoint to acquire this lock and verify it with the UI.

  **Priority:** P1

  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
