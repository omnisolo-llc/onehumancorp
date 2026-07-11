issue_title: "Implement Unified Inventory Booking System with Distributed Locking"
issue_description: |
  # Research Report: Unified Inventory Booking System with Distributed Locking

  ## Problem Statement
  For business owners managing both online and offline sales (like Priya the Boutique Operator or Carlos the Field Service Owner), inventory and booking conflicts are a major source of friction. When an item is sold in-store or a time slot is booked offline, the online availability must update instantly to prevent double-booking. Currently, OHC lacks a real-time, strongly consistent inventory reservation mechanism. Without this, simultaneous purchases across different channels lead to overselling, requiring manual intervention, refunds, and a poor customer experience.

  ## Research Report
  Our competitive analysis shows that traditional platforms like Shopify rely on complex, costly third-party apps to synchronize POS hardware with online storefronts. These integrations often suffer from latency issues, leading to out-of-sync inventory. Advanced systems like Stripe Terminal provide robust payment flows but lack native, agent-driven workflow automation.

  OHC's differentiation lies in its "Assistant-First" approach. We need to implement a centralized inventory and booking synchronization architecture that leverages distributed locks to reserve resources instantly, combined with an Operations Agent that manages the reconciliation and informs the owner.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant C as Customer (Online/Offline)
      participant POS as Mobile POS Client
      participant API as OHC API Layer
      participant RL as Redis Redlock (Lock Manager)
      participant DB as Central Ledger (PostgreSQL)
      participant OA as Operations Agent

      C->>POS: Attempt to purchase/book "Item X"
      POS->>API: Request reservation for "Item X"
      API->>RL: Acquire lock `ohc:lock:{tenant_id}:inventory:{item_id}`
      alt Lock Acquired
          RL-->>API: Lock successful
          API-->>POS: Reservation confirmed (e.g., 5 min hold)
          POS->>API: Complete transaction
          API->>DB: Update inventory/booking status (Row-level lock)
          API->>RL: Release lock
          API->>OA: Notify successful transaction
      else Lock Failed
          RL-->>API: Lock failed (Item reserved/sold)
          API-->>POS: Item unavailable
          API->>OA: Trigger "Item out of stock" alert
      end
  ```

  ### Mobile UX Flow
  1. **User Action:** The owner (e.g., Priya) processes an in-store transaction on her 375px mobile device.
  2. **Reservation State:** The app instantly shows a translucent loading state (macOS-style glass effect) while acquiring the lock.
  3. **Confirmation:** Upon success, a subtle "Reservation Confirmed" notification appears. The customer completes the payment.
  4. **Conflict Resolution:** If a lock fails (e.g., an online customer just reserved the item), the app gracefully notifies the owner: "This item is currently reserved online." The Operations Agent may suggest alternatives.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors failed locks and out-of-stock events. If a popular item is repeatedly conflicting, it drafts a restock order for the owner's approval.
  - **Customer Success Agent:** If an online customer's cart is modified due to a conflict, the agent drafts an apology and an alternative offer.

  ### Key Design Decisions
  - **Redis Redlock:** Used for short-term distributed locks during the checkout/booking process to ensure cross-channel consistency.
  - **PostgreSQL Row-Level Locks:** Used for the final transaction commit to guarantee ACID properties for the central ledger.
  - **Optimistic UI:** The mobile POS client should update optimistically, with clear rollback paths if the reservation fails.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to implement the Unified Inventory Booking System using Redis Redlock for distributed reservation management.

  **Outcome:** A unified inventory system where simultaneous purchases across online and offline channels are safely managed. If an item is being purchased in-store, an online customer attempting to buy the same item will be gracefully notified of its unavailability.

  **Critical User Journey (CUJ):**
  1. An online customer adds "Item X" to their cart and begins checkout, triggering a 5-minute reservation lock in Redis.
  2. The store owner attempts to process an in-store transaction for the same "Item X" via the mobile POS interface.
  3. The POS interface correctly identifies the lock and displays a message indicating the item is currently reserved.
  4. The online customer completes their purchase, permanently deducting the inventory. The POS interface updates to reflect the out-of-stock status.

  **Acceptance Criteria:**
  1. Implement a distributed locking mechanism using Redis (Redlock pattern) for inventory items and booking slots.
  2. Integrate the locking mechanism into the checkout and POS transaction flows.
  3. Ensure the mobile UI gracefully handles lock acquisition failures with clear user messaging.
  4. Add integration tests verifying cross-channel lock contention and resolution.
  5. The implementation must follow OHC's mobile-first UI standards and multi-tenant isolation rules.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
