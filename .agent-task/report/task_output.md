issue_title: "Implement High-Performance Action Queue Architecture for Centralized Inventory Synchronization"
issue_description: |
  # Research Report: High-Performance Action Queue for Centralized Inventory Sync

  ## Problem Statement
  Current e-commerce platforms struggle to keep online inventory in sync with physical Point-of-Sale (POS) transactions in real-time, often leading to double-booking and out-of-stock scenarios. OHC needs a robust, scalable architecture to manage inventory locks across multiple sales channels (online web and offline terminal POS), ensuring that an item reserved in one channel instantly updates availability for the other. Without this, business owners like Priya (Boutique Owner) face dissatisfied customers and operational headaches.

  ## Research Report
  - **Competitive Analysis:** Competitors such as Shopify and Wix often rely on simple eventually consistent polling mechanisms or expensive third-party integrations to manage multi-channel inventory. These systems lack the granular, sub-second locking required for high-velocity flash sales or simultaneous online/in-store traffic.
  - **Industry Best Practices:** High-scale platforms (e.g., Ticketmaster, Stripe) use distributed locks (like Redis Redlock) and asynchronous high-performance job queues to manage transient reservations and ensure strict consistency before final ledger commits.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      ClientPOS[Mobile POS / Terminal] -->|Tap-to-Pay| APILayer
      ClientWeb[Online Storefront] -->|Checkout| APILayer
      APILayer --> DistributedLock[Redis Redlock]
      DistributedLock -->|Acquire Lock| JobQueue[PostgreSQL Job Queue with SKIP LOCKED]
      JobQueue -->|Process Inventory Tx| InventoryLedger[(PostgreSQL Central Ledger)]
      InventoryLedger -->|Sync Status| OpcAgent[Operations Agent - Manager]
      OpcAgent -->|Push Notification| ClientPOS
      OpcAgent -->|Update Cache| ClientWeb
  ```

  ### Mobile UX Flow
  1. A user (e.g., Priya) operates the POS interface on a 375px mobile screen.
  2. When an item is scanned or tapped for checkout, the UI instantly reflects a "Reserving..." state.
  3. The backend attempts to acquire a short-lived Redis lock for that product ID.
  4. If successful, the UI transitions to "Reserved" and proceeds with payment processing.
  5. If the lock fails (item just sold online), the POS displays a graceful "Item Unavailable: Just sold online" message with a 44x44px button to "Dismiss" or "Suggest Alternative".
  6. Simultaneously, the online web client receives real-time updates (via WebSocket/Cache invalidate) disabling the checkout button for that item.

  ### AI Agent Integration
  - **Operations Agent ("The Manager"):** Listens to events from the Job Queue. When a lock collision occurs or an item hits a low-stock threshold, the agent automatically drafts a "Low Stock Alert" or "Restock Proposal" and pushes it to the user's Agent Feed.
  - **Finance Agent:** Correlates the final inventory deduction with the payment ledger entry.

  ### Key Design Decisions
  - **Redis Redlock for Reservation:** Crucial for sub-second locking required during simultaneous checkouts.
  - **PostgreSQL `SKIP LOCKED` Job Queue:** Ensures reliable asynchronous processing of the final inventory decrement and ledger update without locking the user-facing API request.
  - **Multi-Tenant Isolation:** All locks and job queue entries must strictly include the `tenant_id` to guarantee absolute data isolation.

  ## Implementation Prompt
  Implement a robust, centralized inventory synchronization architecture that uses Redis Redlock for temporary inventory reservations during checkout, and a PostgreSQL `SKIP LOCKED` job queue for final ledger commits. The system must gracefully handle lock collisions and push real-time availability updates to both the mobile POS and online storefront. The Operations Agent must be integrated to automatically draft restock proposals when inventory reaches predefined thresholds.

  **Critical User Journey (CUJ):**
  1. Priya is processing an in-store sale on her mobile POS (375px viewport) for the last "Blue Scarf".
  2. An online customer simultaneously attempts to checkout the same "Blue Scarf".
  3. The POS system acquires the Redis lock first. The POS checkout proceeds smoothly.
  4. The online checkout request fails to acquire the lock and gracefully informs the customer that the item is no longer available.
  5. The POS transaction finalizes, the background job queue updates the central ledger, and the Operations Agent pushes a "Blue Scarf sold out" notification to Priya's Agent Feed.

  **Acceptance Criteria:**
  - Redis Redlock is implemented for short-lived inventory reservations during checkout.
  - A PostgreSQL-backed job queue (using `SKIP LOCKED`) processes final inventory deductions.
  - Strict multi-tenant isolation is enforced on all locks and database tables.
  - The Operations Agent correctly generates a restock proposal in the Agent Feed upon item depletion.
  - Mobile UI handles the "Reserving" state and gracefully manages lock failures with touch-friendly elements.
  - High coverage of unit tests and Playwright E2E tests validating the lock collision scenario.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
