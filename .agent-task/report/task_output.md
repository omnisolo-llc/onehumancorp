issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Architecture Implementation Brief: Centralized Inventory & POS Sync

  ## Problem Statement
  Small business operators (like Priya, the boutique owner) need seamless inventory tracking between their online storefronts and in-store Point-of-Sale (POS) systems. Currently, OneHumanCorp (OHC) lacks a real-time, strongly consistent inventory locking mechanism, which leads to out-of-stock and double-booking issues when online and offline transactions occur simultaneously.

  ## Research Findings
  Our gap analysis indicates that while competitors like Shopify offer robust POS integrations, their solutions often fail to meet the needs of micro-SMEs due to complexity and disjointed inventory updates. We need to implement a Redis Redlock-based reservation system and ensure robust PostgreSQL transaction management to provide an integrated, agent-assisted POS experience that seamlessly works both online and offline.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[Online Customer Checkout] -->|Reserve Item| B(Redis Redlock)
      C[POS Client / In-Store] -->|Reserve Item| B
      B -->|Lock Acquired| D{PostgreSQL Central Ledger}
      D -->|Confirm Sale| E[Update Inventory]
      E --> F[Operations Agent]
      F -->|Notify| G[Customer/Owner]
  ```

  ### Entity-Relationship Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ PRODUCT : owns
      PRODUCT ||--o{ INVENTORY_LEDGER : tracks
      TENANT ||--o{ TERMINAL_SESSION : manages
      TERMINAL_SESSION ||--o{ INVENTORY_LEDGER : syncs_to

      PRODUCT {
          uuid id
          uuid tenant_id
          string name
          int total_stock
      }
      INVENTORY_LEDGER {
          uuid id
          uuid product_id
          uuid tenant_id
          int quantity_reserved
          int quantity_sold
          timestamp reserved_at
      }
      TERMINAL_SESSION {
          uuid id
          uuid tenant_id
          string status
          jsonb offline_sync_data
      }
  ```

  ### Mobile UX Flow
  - **Screen 1 (POS Home - 375px):** Priya views her catalog with large, tap-friendly product cards (minimum 44x44px touch targets). She taps the "Red Dress" variant.
  - **Screen 2 (Checkout/Tap-to-Pay):** A translucent glass modal appears confirming the item. At this moment, a Redis Redlock is applied. Priya prompts the customer to tap their card on the Stripe Terminal.
  - **Screen 3 (Concurrent Online User):** If an online user is currently viewing their cart with the "Red Dress," the Operations Agent triggers a graceful, soft UI update: "Item just sold out. Please remove to continue."
  - **Screen 4 (Confirmation):** The in-store tap-to-pay completes. The lock is released, PostgreSQL is permanently decremented. Priya receives a dashboard notification to restock.

  ### AI Agent Integration
  The **Operations Agent** actively monitors the `INVENTORY_LEDGER`. It is responsible for orchestrating the notification when a Redis lock becomes a permanent sale, alerting online customers, and prompting Priya to order new inventory when the `total_stock` falls below her designated threshold.

  ## Implementation Prompt
  **Target Outcome:** Build the OHC Unified Multi-Channel Inventory Sync & POS system.
  **CUJ to support:** Priya is logged into the POS mobile app. She processes an in-store sale for an item. The system must apply a Redis lock (e.g., 15 seconds) to reserve the item. If an online customer tries to buy the same item concurrently, they receive a sold-out message. The final transaction updates the PostgreSQL ledger and the Operations Agent notifies Priya.

  **Acceptance Criteria:**
  1. Implement Redis Redlock inventory reservation service.
  2. Integrate the lock mechanism into the checkout/POS flow.
  3. Ensure robust data schema handling for `TerminalSession` with offline-sync reconciliation.
  4. Extend the Operations Agent for low-stock alerts and restock suggestions.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
