issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  ## Problem Statement
  Small business owners like Priya (boutique operator) need a unified inventory system that seamlessly integrates online (web/mobile) and offline (in-store tap-to-pay) sales. Disjointed systems cause double-booking and out-of-stock scenarios, frustrating both the business owner and customers. OHC lacks a real-time, robustly consistent inventory locking and caching system to handle multi-channel operations effectively.

  ## Research Report
  - **Market Context:** Traditional platforms like Shopify rely on costly third-party integrations or enterprise plans for true real-time offline POS synchronization. Systems like Square have robust POS but lack unified, AI-driven operations for the entire business.
  - **The OHC Opportunity:** Implement a zero-configuration, robust inventory reservation system and eventual consistency mechanism for offline POS that just works for non-technical users. The Operations Agent can manage out-of-stock scenarios intuitively without manual owner intervention.

  ## Design Doc
  ### Data Model & Sync Protocol (Architecture Diagram)
  ```mermaid
  graph TD;
      A[In-Store Tap-to-Pay] -->|POS Sync Event| B(Redis Redlock: Reserve Inventory)
      C[Online Customer Checkout] -->|WebCart| B
      B -->|Lock Success| D[PostgreSQL Central Ledger]
      B -->|Lock Fail - Out of Stock| E[Operations Agent: Trigger Notification]
      D --> F[Finance Agent: Sync Invoices]
      A -.->|Offline Mode| G[Local Caching: Eventual Consistency]
      G -.->|Network Restored| D
  ```
  - **Central Ledger (PostgreSQL):** Source of truth. Uses optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock):** `ohc:lock:{tenant_id}:inventory:{product_id}`. Short TTL (15s for POS, 5m for online carts).
  - **Offline Client:** Mobile POS caches product catalog and uses eventual consistency to sync finalized offline sales when online.

  ### Mobile UX Flow (375px)
  1. Owner logs into OHC app (POS mode). Clear catalog with large (44x44px) touch targets.
  2. Owner processes tap-to-pay. Redis locks the inventory item for 15s.
  3. UI optimistically updates inventory.
  4. If out of stock, owner gets a "Red Dress sold out. Draft restock order?" push notification via the Operations Agent.

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager"):** Actively monitors stock. Triggers low-stock alerts, manages sync conflicts if offline synchronization encounters issues, and suggests restocks.
  - **Customer Success Agent ("The Ambassador"):** Updates storefront availability and notifies online customers if items in carts become unavailable.

  ## Implementation Prompt
  **User-Facing Outcome:** Priya the boutique owner can sell the last "Red Dress" in-store using her phone's tap-to-pay, and an online customer trying to check out at the exact same moment gets a graceful "Item just sold out" message.
  **CUJ & Acceptance Criteria:**
  1. Create the Redis Redlock inventory reservation service and integrate it with the checkout endpoints.
  2. Define the schema and sync endpoints to handle offline-first eventual consistency with PostgreSQL for point of sale transactions.
  3. Ensure that when a Redis lock fails, the Operations Agent is triggered to issue low-stock or out-of-stock push notifications.
  4. Write Playwright E2E tests validating the checkout collision scenario (one successful POS transaction blocking an online checkout attempt).

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
