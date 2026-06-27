issue_title: "Implement Distributed Inventory Caching & Redlock Mechanism"
issue_description: |
  ## Issue Brief: OHC Unified Multi-Channel Inventory Sync & POS

  ### Problem Statement
  Priya, our boutique owner persona, requires flawless inventory tracking between her online store and in-person tap-to-pay sales. Currently, OHC lacks a robust distributed lock and synchronization architecture. This gap results in potential double-booking or out-of-stock scenarios during simultaneous online checkout and in-store point-of-sale transactions. A centralized, strongly consistent inventory system with transient distributed locks is required to maintain the Single Source of Truth while providing instantaneous POS performance.

  ### Research Report
  Our competitive analysis indicates that micro-SMEs frequently struggle with siloed inventory states (e.g., Shopify POS vs Web requires robust network access or costly third-party integrations). By integrating a localized eventual consistency model via a Redis Redlock pattern, we can provide Priya with a resilient and synchronized inventory ledger. The lock prevents online checkout of the last item if an in-store tap-to-pay transaction is currently processing, guaranteeing that stock limits are strictly enforced.

  ### Design Doc
  **Architecture Overview:**
  - **Central Source of Truth:** PostgreSQL (inventory ledger).
  - **Distributed Lock Mechanism:** Redis Redlock (temporary reservation during active checkout/POS transaction).
  - **Lock Pattern:** `ohc:lock:{tenant_id}:inventory:{product_id}`.

  **Mobile UX Flow (375px Base):**
  - The POS mobile UI (Flutter/PWA) must clearly indicate item availability.
  - Upon tapping to pay, the system attempts to acquire a Redis lock. If successful, the UI transitions to the payment state; if it fails (e.g., someone just checked out online), the UI gracefully rolls back with a clear "Item just sold out" notification.
  - All interactive elements must adhere to the 44x44px minimum touch target requirement and use the premium translucent glass design language.

  **AI Agent Integration:**
  - **Operations Agent ("The Manager"):** Monitors inventory levels. When an item stock drops to zero or triggers a predefined low-stock threshold due to a POS sale, this agent proactively queues a push notification suggesting a restock order.

  ### Implementation Prompt
  As an implementer agent, your objective is to:
  1.  **Develop the Redlock Service:** Implement the Redis Redlock pattern within the Go backend.
  2.  **Integrate with Checkout/POS Flow:** Ensure that the checkout APIs (both online and POS) attempt to acquire this lock before mutating the database ledger.
  3.  **Implement Optimistic UI Updates & Error Handling:** Update the POS UI in Flutter (handling a 375px viewport) to gracefully manage lock acquisition failures (e.g., "Item just sold out").
  4.  **Extend Operations Agent:** Add logic for the Operations Agent to detect stock depletions and enqueue a restock suggestion notification.
  5.  **Verify & Test:** Ensure comprehensive unit test coverage for the lock service and implement a Playwright E2E test verifying the concurrent checkout lock mechanism. Ensure all UI elements use OHC design tokens.

  **Note:** Do not prescribe specific SQL schemas or precise function signatures; design the solution to meet the architectural and UX goals defined above. Ensure complete adherence to the OHC core engineering standards and superpowers workflows.

  ### Priority
  P1

  ### Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
