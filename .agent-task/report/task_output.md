issue_title: "Implement Real-time Inventory Distributed Locking for POS"
issue_description: |
  # Research Report: Real-time Inventory Sync & Distributed Locking for Offline POS Operations

  ## Problem Statement
  Currently, OHC lacks a real-time, strongly consistent inventory locking mechanism for merchants operating across multiple channels (online and in-store). If a physical customer and an online customer attempt to purchase the last unit of an item simultaneously, the system risks double-booking or selling out-of-stock items. Priya, our boutique owner persona, needs to trust that her online inventory accurately reflects her physical stock without manual reconciliation or expensive third-party tools.

  ## Research Report
  Our competitive analysis indicates that while major platforms like Shopify provide extensive POS capabilities, their inventory management for micro-SMEs often remains disjointed. Online inventory frequently falls out-of-sync with in-person sales unless costly tools or higher-tier plans are employed. Local-first tools like Square provide robust POS hardware but lack integrated, agentic workflow automation.

  The gap in OHC's platform is the absence of a distributed locking and synchronization protocol that bridges the offline/online gap seamlessly. We need a reliable mechanism to hold inventory during checkout (whether online or via a POS terminal) and synchronize offline sales when the network is restored.

  ## Design Doc
  ### Architecture
  1.  **Distributed Lock Manager (Redis Redlock):** Implement a robust lock mechanism utilizing Valkey/Redis. This will reserve inventory temporarily during the checkout process.
  2.  **Lock Key Pattern:** `ohc:lock:{tenant_id}:inventory:{product_id}`.
  3.  **Lock Durations:** Differentiate lock durations based on the channel (e.g., shorter locks for rapid tap-to-pay in-store transactions, longer locks for online shopping carts).
  4.  **Local-First POS Cache & Sync:** The mobile POS client (Frontend) will cache catalog data and use optimistic UI updates. It will require an eventual consistency mechanism to synchronize finalized offline sales and reconcile with the central PostgreSQL ledger.
  5.  **Conflict Resolution:** Implement logic to handle scenarios where a lock cannot be acquired or an offline sync encounters a conflict.

  ```mermaid
  sequenceDiagram
      participant C as POS Client (Mobile)
      participant O as Online Client
      participant S as Backend API
      participant R as Redis (Valkey)
      participant DB as Central Ledger (PostgreSQL)

      C->>S: Request Inventory Lock (Product A)
      S->>R: Acquire Lock: ohc:lock:tenant:inventory:A
      R-->>S: Lock Acquired
      S-->>C: Lock Confirmed

      O->>S: Request Inventory Lock (Product A)
      S->>R: Acquire Lock: ohc:lock:tenant:inventory:A
      R-->>S: Lock Denied (Already held)
      S-->>O: Error: Item unavailable

      C->>S: Finalize Checkout (Product A)
      S->>DB: Update Inventory Ledger
      S->>R: Release Lock
  ```

  ### AI Agent Integration
  -   **Operations Agent:** Monitors inventory levels and resolves sync conflicts.
  -   **Customer Success Agent:** Notifies online customers if an item in their cart becomes unavailable due to an in-store purchase.

  ### Mobile UX Flow (375px)
  1.  The user taps an item to add to the cart.
  2.  The UI instantly shows the item in the cart (optimistic update).
  3.  A subtle loading indicator shows the reservation in progress.
  4.  If the lock fails (item out of stock), a clear, non-technical error message appears ("Oops, this item just sold out!"), and the item is removed from the cart.

  ## Implementation Prompt
  **Goal:** Implement a distributed inventory locking mechanism using Redis (Valkey) to prevent double-booking during multi-channel checkouts.

  **Tasks:**
  1.  Implement a lock acquisition and release mechanism in the backend services using Redis. Use the defined key pattern.
  2.  Expose API endpoints for reserving inventory (acquiring lock) and completing the sale (updating DB and releasing lock).
  3.  Update the frontend POS interface to handle these new endpoints, including optimistic UI updates and error handling for failed reservations.
  4.  Ensure the solution handles potential race conditions and network latency.

  **Acceptance Criteria:**
  -   Simultaneous checkout attempts for the last item across different sessions result in only one successful reservation.
  -   The POS interface gracefully handles reservation failures.
  -   Locks expire automatically if a transaction is abandoned.

  ## Priority
  P1 (High)

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
