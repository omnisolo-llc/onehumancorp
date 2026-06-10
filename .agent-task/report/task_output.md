issue_title: "Implement Real-time Multi-Channel Inventory Sync & POS Lock Architecture"
issue_description: |
  # Research Report: Real-time Multi-Channel Inventory Sync & POS Lock Architecture

  ## Problem Statement
  Small business owners like Priya (Boutique Operator) sell across multiple channels simultaneously (e.g., in-store Point-of-Sale via tap-to-pay and an online web storefront). Currently, OHC lacks a robust, real-time distributed locking and inventory synchronization mechanism. This leads to a critical business failure: double-booking and out-of-stock scenarios when simultaneous online and offline purchases occur for the same physical item. The current system cannot guarantee strong consistency across these disparate sales channels without technical intervention, violating our core promise of radical simplicity.

  ## Research Report
  - **Competitor Landscape:**
    - *Shopify:* Offers strong POS integration and inventory sync, but struggles to serve micro-SMEs without pushing them towards expensive third-party apps or complex higher-tier plans.
    - *Square/Stripe Terminal:* Provide excellent hardware and payment flows but lack the deep, AI-driven agentic workflow integration necessary for proactive operations management.
  - **OHC Opportunity:** By leveraging Redis Redlock for distributed locking across `tenant_id` scopes and combining it with our asynchronous Operations Agent, OHC can provide a zero-configuration, invisible synchronization layer. This ensures that an in-store transaction instantly reserves the item, preventing simultaneous online purchases, while the AI agent proactively manages low-stock notifications and restock workflows.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      actor Customer
      actor Priya (POS)
      participant OHC Backend
      participant Redis (Redlock)
      participant Postgres (Ledger)
      participant Operations Agent

      Customer->>OHC Backend: Add to Cart (Item X)
      Priya (POS)->>OHC Backend: Initiate Tap-to-Pay (Item X)

      Note over OHC Backend, Redis (Redlock): Concurrent Reservation Attempt
      OHC Backend->>Redis (Redlock): Acquire Lock: `ohc:lock:{tenant_id}:inventory:{item_id}`
      Redis (Redlock)-->>OHC Backend: Lock Granted (15s TTL)

      OHC Backend->>Priya (POS): Proceed to Payment
      OHC Backend->>Customer: Alert: Item Reserved by Another Customer

      Priya (POS)->>OHC Backend: Payment Success
      OHC Backend->>Postgres (Ledger): Commit Inventory Deduction
      OHC Backend->>Redis (Redlock): Release Lock

      OHC Backend->>Operations Agent: Event: Inventory Updated (Item X)
      Operations Agent->>Operations Agent: Check Low-Stock Threshold
      Operations Agent->>Priya (POS): Notification: Action Card "Restock Item X?"
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **POS Checkout Screen (In-Store):** A clean, un-cluttered screen featuring large touch targets (≥ 44x44px) for adding items to the cart and a prominent "Charge" button. When tapped, the UI immediately shows a "Reserving..." state before opening the tap-to-pay interface, utilizing optimistic UI principles with rollback if the lock fails.
  - **Online Storefront (Customer):** Standard product listing. If an item is locked via POS, the UI gracefully transitions the "Add to Cart" button to a disabled "Reserved" or "Out of Stock" state based on real-time websocket/polling updates.
  - **Agent Feed (Owner):** If an item drops below a critical threshold due to a sale, the Operations Agent pushes an Action Card to the owner's 375px viewport: "Item X is low in stock. Draft a restock order?" with "Approve" and "Dismiss" buttons.

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager"):** Listens to inventory mutation events on the message bus. Triggers low-stock alerts and drafts supplier restock emails automatically based on velocity and thresholds.
  - **Customer Success Agent ("The Ambassador"):** Updates the online storefront availability and can proactively notify waitlisted customers if a reserved item becomes available again (e.g., failed payment).

  ### Key Design Decisions
  - **Redis Redlock:** Chosen for high-performance, short-lived distributed locking during the critical checkout phase to prevent double-booking across distributed nodes.
  - **Tenant-Scoped Locks:** Lock keys must follow the strict pattern `ohc:lock:{tenant_id}:inventory:{resource_id}` to guarantee multi-tenant isolation.
  - **Event-Driven AI:** Inventory updates are published to the message bus rather than synchronously calling agents, ensuring checkout latency remains low.

  ## Implementation Prompt
  **Target Persona:** Priya the Boutique Owner
  **Outcome:** Implement the backend distributed locking mechanism and the mobile POS checkout flow to ensure real-time inventory synchronization between online and in-store sales channels.
  **Critical User Journey (CUJ):**
  1. Priya logs into the OHC mobile app and enters POS mode.
  2. An online customer views the same item on the web storefront.
  3. Priya initiates checkout for the item in-store.
  4. The system successfully acquires a distributed lock in Redis, preventing the online customer from checking out the identical item.
  5. Upon payment success, the inventory is permanently deducted in the central ledger (Postgres).
  **Acceptance Criteria:**
  - Introduce a Redis Redlock mechanism in the Rust backend specifically for inventory reservation during checkout.
  - Ensure strict tenant isolation in all lock keys and database queries.
  - Update the mobile POS UI (Flutter/Tauri) to handle the reservation state and display appropriate loading/error feedback.
  - Write Playwright E2E tests simulating simultaneous checkout attempts from a POS client and a web client, verifying that only one succeeds and the other receives a proper rejection.
  - Implement at least 5 unit tests covering the lock acquisition, release, and expiration logic.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []