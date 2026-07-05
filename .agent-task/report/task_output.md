issue_title: "[research] Architect and Implement Real-Time Offline-First Inventory and POS Synchronization"
issue_description: |
  **Problem Statement:**
  Small business owners like Priya (boutique operator) manage inventory across physical in-store locations and an online storefront. Existing platforms like Shopify or Wix struggle to maintain real-time inventory synchronization without expensive third-party plugins. A customer buying the last item in-store often leads to an online double-booking if the inventory doesn't sync instantaneously, leading to poor customer experience and operational overhead.

  **Research Report:**
  - **Market Context:** Most competitors (Shopify, Wix, Square) separate online and in-store inventory logic, attempting to sync them through eventual consistency models that often fail under high concurrency.
  - **OHC Opportunity:** By utilizing our centralized PostgreSQL ledger and Redis Redlock distributed locking mechanism, we can create an instantaneous reservation system. This ensures that the moment a physical POS transaction begins, the item is locked online, preventing double bookings.
  - **Competitor Gaps:**
    - *Shopify:* High latency in inventory updates across channels; often requires premium plans for advanced inventory features.
    - *Square:* Excellent offline POS but struggles with instantaneous online storefront synchronization.

  **Design Doc:**
  - **Architecture Diagram:**
    ```mermaid
    graph TD;
        A[Mobile POS Client 375px] -->|Initiate Sale| B(Redis Redlock Reservation)
        C[Online Storefront] -->|Add to Cart| B
        B -->|Lock Success| D[PostgreSQL Inventory Ledger]
        B -->|Lock Failure| E[Return Out of Stock]
        D --> F[Operations Agent: Trigger Restock/Alert]
    ```
  - **Data Model:** Centralized `inventory_ledger` and `inventory_reservations` in PostgreSQL with row-level security.
  - **Mobile UX Flow (375px):**
    - **POS View:** Priya scans/selects an item. The system instantly requests a Redis lock. If successful, the item is added to the POS cart.
    - **Online View:** A customer viewing the item sees "1 left". If Priya's POS acquires the lock, the online view dynamically updates (via WebSocket/SSE) to "In another customer's cart" or "Sold Out".
  - **AI Integration:** The Operations Agent monitors stock levels and lock failures. If an item frequently fails to lock (high demand), the agent drafts a reorder proposal for Priya.

  **Implementation Prompt:**
  **Target Persona:** Priya the Boutique Operator
  **Outcome:** Priya can confidently sell her last "Red Dress" in-store without fearing an online customer double-booking it.

  **Next Actions:**
  1. Implement a distributed locking service using Redis (`ohc:lock:{tenant_id}:inventory:{product_id}`).
  2. Integrate the locking mechanism into the checkout and POS cart addition flows.
  3. Develop a WebSocket/SSE notification system to update online storefront availability in real-time when a lock is acquired.
  4. Extend the Operations Agent to track lock contention and suggest restocking.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
