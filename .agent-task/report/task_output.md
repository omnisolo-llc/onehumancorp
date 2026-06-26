issue_title: "Implement Offline-Tolerant Local POS Client Architecture"
issue_description: |
  # Research Report: Offline-Tolerant Local POS Client Architecture

  ## Problem Statement
  Small business owners like Fatima (food cart operator) and Carlos (handyman) often operate in environments with flaky, slow, or nonexistent internet connections (e.g., outdoor events, rural homes, basements). Currently, OHC relies entirely on real-time internet connectivity for its POS and inventory operations. When the connection drops, they cannot process in-person sales, update their catalog, or sync inventory, completely halting their business operations. They need an offline-tolerant POS client that works locally and syncs back when connectivity is restored, without requiring them to understand "sync conflicts" or "eventual consistency."

  ## Research Report
  - **Competitive Landscape**:
    - *Square*: Provides an "Offline Mode" that allows processing card payments (at the merchant's risk) and cash sales. It queues transactions locally and syncs them automatically when reconnected.
    - *Shopify POS*: Has limited offline capabilities. Cash transactions and basic catalog browsing work, but many features require a connection.
    - *OHC Opportunity*: Go beyond basic transaction queuing. Implement a robust offline-first architecture using a local database (e.g., SQLite or IndexedDB via Flutter/PWA) combined with Powersync (already in docker-compose) or a custom CRDT/eventual consistency mechanism. The Operations Agent should handle any edge cases (like negative inventory from offline sales) silently in the background when syncing.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile POS App - Flutter/PWA] --> B(Local SQLite/IndexedDB)
      B --> C{Sync Engine - PowerSync/Custom}
      C -- "Offline" --> B
      C -- "Online" --> D[OHC Backend API]
      D --> E[PostgreSQL Ledger]
      D --> F[Redis Distributed Locks]
      D --> G[Operations Agent]
      G -->|Reconcile Conflicts| E
  ```

  ### Mobile UX Flow (375px)
  1. **Connectivity Indicator**: A subtle, non-intrusive indicator (e.g., a small cloud icon with a line through it) shows when the device is offline.
  2. **Offline Transaction**: Fatima taps items on the POS menu. The app responds instantly. She selects "Cash" or an "Offline Card" payment method. The sale completes immediately.
  3. **Sync Feedback**: When connectivity returns, a small toast notification says "Syncing 3 sales..." and then "All caught up!"
  4. **Agent Resolution**: If an item oversold while offline, Fatima receives a feed item: "We sold 2 extra Lemonades while offline. I've updated the inventory and drafted a restock note."

  ### Key Design Decisions
  - **Local First**: All reads (catalog) and writes (POS sales, inventory deductions) hit the local database first to ensure <50ms latency regardless of network conditions.
  - **Background Sync**: Syncing happens asynchronously. The user is never blocked waiting for a network request to complete.
  - **Agentic Conflict Resolution**: Instead of showing scary error messages about database conflicts, the Operations Agent is responsible for parsing sync discrepancies (e.g., inventory going below zero) and proposing plain-language resolutions to the owner.

  ## Implementation Prompt
  **Feature Name**: Offline-Tolerant POS & Inventory Sync

  **Target Persona**: Fatima the Food Cart Operator

  **Outcome**: Fatima can continue taking orders and processing cash sales even when her mobile data drops out. The system automatically syncs her sales to the central ledger when she regains connection, and the Operations Agent gracefully handles any inventory discrepancies (like selling more than the central ledger thought was available).

  **Critical User Journey (CUJ)**:
  1. Start the OHC POS app and ensure the catalog is loaded.
  2. Disconnect the device from the internet (simulate offline mode).
  3. Process a cash sale for an item. Verify the sale completes instantly and the local inventory count decreases.
  4. Reconnect the device to the internet.
  5. Verify that the background sync process pushes the offline transaction to the backend.
  6. Verify the central PostgreSQL ledger reflects the sale and the correct inventory count.

  **Next Actions**:
  - Implement a robust local database integration (e.g., using `sqflite` or similar for Flutter, or a robust IndexedDB wrapper for PWA) to cache the product catalog and queue offline transactions.
  - Develop the bi-directional sync engine that pushes queued transactions to the backend and pulls updated catalog/inventory data.
  - Update the backend API to handle bulk/offline transaction sync requests, ensuring idempotency.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
