issue_title: "Implement Real-Time Multi-Channel Inventory Sync & Distributed POS"
issue_description: |
  ## Title
  Real-Time Multi-Channel Inventory Sync & Distributed POS Architecture

  ## Problem Statement
  Small business owners like Priya (boutique owner) require seamless inventory tracking between their online storefronts and in-store operations (tap-to-pay or card readers). Currently, the system lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, simultaneous online and offline purchases often result in double-booking, stockouts, and dissatisfied customers, forcing the owner to manually reconcile inventory across systems. Competitor solutions often require costly third-party integrations or enterprise-tier plans to achieve true synchronization.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify POS:** Dominates e-commerce with extensive POS capabilities, but the complexity and cost of higher-tier plans often alienate micro-SMEs. Inventory can sometimes fall out-of-sync without expensive third-party tools.
  - **Square:** Offers robust POS hardware and software, but its integration with online storefronts can be disjointed unless the user fully commits to the Square ecosystem, lacking the autonomous agentic workflow needed by non-technical operators.
  - **Stripe Terminal:** Provides excellent developer APIs for POS but requires significant custom development to build a unified, agent-driven inventory system.
  - **OHC Opportunity:** OHC must provide a completely seamless, real-time inventory synchronization engine natively. By combining a centralized PostgreSQL ledger, Redis-based distributed locking (Redlock), and an offline-capable mobile POS client, OHC can guarantee consistency. The Operations Agent ("The Manager") will autonomously monitor stock levels, handle edge-case reconciliations, and notify the owner of critical stock events, providing a zero-configuration, enterprise-grade experience to solopreneurs.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Online Customer Checkout] -->|API Request| B(Inventory Service)
      C[In-Store POS / Stripe Terminal] -->|API Request| B
      B --> D{Redis Redlock Reservation}
      D -->|Lock Acquired| E[Process Transaction]
      D -->|Lock Denied| F[Return 'Out of Stock' Error]
      E --> G[PostgreSQL Central Ledger Update]
      G --> H[Operations Agent / The Manager]
      H -->|Low Stock Alert| I[Mobile App Feed 375px]
      C -.->|Offline Mode| J[Local POS Cache]
      J -.->|Sync when Online| B
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **POS Checkout Screen (Mobile):** Large, tap-friendly product catalog (≥ 44x44px touch targets). When a product is selected and checkout begins, a subtle UI indicator shows the item is "Locked for Purchase."
  - **Online Storefront:** If an item is locked by an in-store transaction, the online storefront automatically updates the 'Add to Cart' button to 'Temporarily Unavailable' or 'Out of Stock' based on the lock status.
  - **Agent Notification:** If a transaction finalizes and reduces stock below the threshold, a card appears in the owner's feed: "Red Dress sold out. Would you like to draft a restock order?" with 1-tap approval buttons.

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager"):** Monitors inventory levels continuously. If an item sells out due to an in-store purchase, it automatically triggers updates to the online storefront and alerts the owner. It also analyzes sales velocity to predict future stockouts and suggest reorder quantities.

  ### Key Design Decisions
  - **Distributed Locking:** Implement Redis Redlock to handle temporary inventory reservations during the checkout flow (e.g., 5-minute lock for online carts, 15-second lock for rapid in-store tap-to-pay).
  - **Central Ledger:** PostgreSQL remains the absolute source of truth, utilizing row-level locks or optimistic concurrency control for final inventory deductions.
  - **Offline/Local First POS:** The mobile POS client must cache product catalogs locally. If the network drops, it should queue transactions and employ eventual consistency to sync with the central ledger upon reconnection, prioritizing in-store sales over pending online carts where conflict resolution is necessary.

  ## Implementation Prompt
  **User-Facing Outcome:** As Priya, the boutique owner, when I sell the last "Red Dress" to an in-store customer using tap-to-pay, the item is instantly marked out of stock on my online website. If an online customer tries to buy it at the exact same moment, they are gracefully informed it just sold out. I don't have to do anything; the Operations Agent handles it and simply sends me a notification suggesting I reorder the dress.
  **CUJ & Acceptance Criteria:**
  1. Implement a distributed locking service (e.g., using Redis Redlock) to reserve inventory quantities for a specified duration during checkout initiation.
  2. Integrate the locking service into both the online checkout flow and the POS terminal flow.
  3. Ensure final transaction processing correctly deducts the reserved quantity from the PostgreSQL central ledger.
  4. Develop the Operations Agent logic to monitor stock levels post-transaction and generate appropriate notifications or auto-replenishment drafts.
  5. Provide Playwright E2E tests: A test simulates concurrent checkout attempts for a single-stock item from both an online client and a POS client. The test must verify that only one transaction succeeds, the other receives an appropriate out-of-stock message, and the final inventory count is zero without negative balances.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
