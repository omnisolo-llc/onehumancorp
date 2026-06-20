issue_title: "[Platform Architecture] Implement Centralized Inventory & Offline-First Distributed POS Lock Sync"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Title
  Implement Centralized Inventory & Offline-First Distributed POS Lock Sync

  ## Problem Statement
  Small business owners with hybrid sales models (e.g., Priya running an in-store boutique and an online web store) face critical issues with inventory syncing. When a customer purchases an item in-store via a POS card reader while another customer concurrently buys the last item online, current systems fail to reconcile stock in real-time, resulting in double-booking, over-selling, and manual refund headaches. OHC needs an architecture that seamlessly locks and syncs inventory across online channels and mobile-first offline Point-of-Sale (POS) systems.

  ## Research Report
  - **Competitor Landscape**: Shopify POS provides decent sync but requires an always-online connection, and its offline mode can lead to eventual consistency conflicts. Square dominates offline but online integrations are clunky. Durable and AI-native builders lack physical inventory POS support entirely.
  - **Market Need**: A recent analysis of r/smallbusiness and app store reviews highlights "Inventory Sync" as a top 10 pain point (12%), with users complaining: "Sold out online but still in store."
  - **Technical Gap in OHC**: OHC currently lacks an edge-capable, distributed locking mechanism for inventory that can handle transient network failures from mobile POS clients (e.g., a 375px Android device used in a pop-up shop).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      MobilePOS[Mobile POS - 375px Offline First] -->|Async Sync/Eventual Consistency| EdgeAPI[Edge Sync API]
      OnlineStoreFront[Online Storefront] -->|Sync Reservation| EdgeAPI
      EdgeAPI -->|Acquire Inventory Lock| Redis[Redis Redlock Cluster]
      EdgeAPI -->|Commit Final Transaction| Postgres[PostgreSQL Central Ledger]
      Postgres -->|Trigger Update| InventoryAgent[Operations AI Agent]
      InventoryAgent -->|Notify Customer/Owner| UnifiedInbox[OHC Unified Feed]
  ```

  ### UI Wireframes & Screen Flow (375px First)
  1. **POS Home (375px)**: A clean, UniFi-style glass-morphic grid of top-selling items. Tap to add to cart. Touch targets are large (minimum 44x44px).
  2. **Checkout Sheet**: A translucent bottom sheet slides up. Displays total and a prominent "Tap to Pay" button.
  3. **Sync Indicator**: A small, unobtrusive status token at the top right showing "Online" (green dot) or "Syncing..." (yellow dot), ensuring the owner always knows the network state without technical jargon.

  ### Mobile UX Flow
  - The owner taps an item to sell. The local client instantly reduces local cached inventory (Optimistic UI).
  - A background sync queue attempts to acquire a short-lived Redis lock for the transaction.
  - If successful, the PostgreSQL ledger is updated, and the Redis lock is released.
  - If offline, the transaction is queued locally. Once reconnected, a batch sync reconciles with the server. If an over-sell occurred, the Operations AI Agent generates an alert card in the Unified Inbox proposing a resolution (e.g., source from another location or refund).

  ### AI Agent Integration Points
  - **Operations Agent**: Monitors the PostgreSQL ledger for low-stock thresholds or conflict events (e.g., offline over-sell). Automatically drafts a reorder proposal or a customer apology email if an online order cannot be fulfilled due to a prior offline sale.
  - **Sales Agent**: Observes fast-moving items and suggests dynamic bundle offers directly to the owner's dashboard.

  ### Key Design Decisions
  - **Redis Redlock**: Chosen for distributed, low-latency reservation locks during the checkout window to prevent double-spends.
  - **Optimistic Concurrency**: The PostgreSQL ledger uses strict versioning (`version` column) to reject stale updates from delayed offline syncs.
  - **Offline-First App Architecture**: Built into the Tauri/Flutter client to ensure Priya can sell at a farmer's market even with poor mobile data.

  ## Implementation Prompt
  **Context**: The backend needs a distributed inventory locking mechanism and an eventual-consistency sync endpoint to support hybrid online/in-store sales.
  **Task**:
  1. Implement a Redis-backed distributed lock mechanism (using the Redlock algorithm pattern) for inventory items. Lock keys must follow `ohc:lock:{tenant_id}:inventory:{product_id}`.
  2. Create an API endpoint `POST /api/v1/inventory/sync` that accepts batch transaction events from offline POS clients.
  3. Update the PostgreSQL schema to include a `version` column on inventory records for optimistic concurrency control.
  4. Ensure all endpoints enforce row-level tenant isolation (`tenant_id`).
  **Acceptance Criteria**:
  - The API correctly acquires and releases Redis locks during concurrent purchase attempts.
  - The API handles stale `version` updates by returning a conflict error, which the client can resolve.
  - Unit tests provide 100% coverage of the locking and synchronization logic.
  - Playwright E2E tests simulate a concurrent online checkout and an offline POS sync, verifying the correct inventory tally and conflict resolution.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
