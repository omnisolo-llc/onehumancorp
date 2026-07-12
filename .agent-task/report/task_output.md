issue_title: "OHC Unified Multi-Channel Inventory Sync & Distributed POS Architecture"
issue_description: |
  ## Problem Statement
  Small business owners (like Priya, the boutique operator) struggle to maintain synchronized inventory across their online storefronts and in-store Point-of-Sale (POS) systems. Without robust synchronization, dual-channel selling leads to overselling, double-booking, and lost trust. Current platforms (Shopify, Wix) often require complex, expensive third-party plugins to achieve this, leaving non-technical SMBs frustrated and vulnerable to inventory chaos.

  ## Research Report
  - **Market Context**: According to the `ohc_smb_market_report.md` and `[research]_ohc_centralized_inventory_pos.md`, "Inventory Sync" is a Top 10 SMB pain point (12%). SMBs frequently face the "Sold out online but still in store" dilemma.
  - **Competitor Analysis**: While Shopify offers POS hardware, strong real-time inventory synchronization often demands higher-tier plans or disjointed plugins. Square and Stripe Terminal are great for physical checkout but lack the agentic workflows needed to unify the online and offline business invisibly.
  - **The OHC Differentiator**: OHC will implement a seamless, multi-channel inventory system backed by a real-time distributed locking mechanism. Crucially, the "Operations Agent" will invisibly manage conflicts, notify customers, and suggest restock orders without requiring the user to navigate complex administrative dashboards.

  ## Design Doc
  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL)**: Serves as the ultimate source of truth, utilizing row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock)**: Used to create temporary inventory reservations during checkout (e.g., 5 mins online, 15 secs for tap-to-pay) to prevent double-booking.
  - **Offline/Local First POS Client**: The mobile POS client caches catalog data and syncs finalized sales via eventual consistency when the network restores.

  ### AI Agent Coordination
  - **Operations Agent ("The Manager")**: Monitors stock, manages sync conflicts, and triggers low-stock alerts/restock drafts.
  - **Customer Success Agent ("The Ambassador")**: Updates online storefront availability and handles graceful "just sold out" notifications for concurrent shoppers.
  - **Finance Agent ("The Accountant")**: Correlates POS terminal transactions with online sales for unified reporting.

  ### Mobile-First Implementation
  - POS mode must be perfectly usable on a 375px viewport with native mobile keyboards.
  - Touch targets for core inventory adjustment and checkout actions must be ≥ 44x44px.
  - Optimistic UI updates with rollback handling for flaky networks.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer (Online)
      participant Priya (In-Store POS)
      participant OHC API (App Shell)
      participant Redis (Redlock)
      participant PostgreSQL (Ledger)
      participant Ops Agent

      Priya (In-Store POS)->>OHC API: Initiates Tap-to-Pay Checkout (Item A)
      OHC API->>Redis (Redlock): Acquire 15s lock on Item A
      Redis (Redlock)-->>OHC API: Lock Acquired
      Customer (Online)->>OHC API: Attempts Add to Cart (Item A)
      OHC API->>Redis (Redlock): Check lock status
      Redis (Redlock)-->>OHC API: Lock Active (In-Store)
      OHC API-->>Customer (Online): Show "Item just sold out" (via Ambassador Agent)
      Priya (In-Store POS)->>OHC API: Payment Success
      OHC API->>PostgreSQL (Ledger): Deduct Item A from Inventory
      OHC API->>Redis (Redlock): Release Lock
      OHC API->>Ops Agent: Trigger Low Stock Check
      Ops Agent-->>Priya (In-Store POS): Push Notification: "Item A sold out. Draft restock order?"
  ```

  ## Implementation Prompt
  Implement the Unified Multi-Channel Inventory Sync & POS architecture. Focus on the core infrastructure:
  1. Set up the Redis Redlock reservation mechanism for inventory items during checkout.
  2. Implement the `TerminalSession` data schema in PostgreSQL to handle offline-sync reconciliation.
  3. Wire up the Operations Agent to monitor these transactions, handle the sync conflicts (triggering the Ambassador Agent if an online user is blocked), and push low-stock notifications.
  4. Ensure the POS checkout UI is 375px-friendly, with optimistic UI updates for inventory changes.

  **Estimated Scope**: Large

  **Acceptance Criteria**:
  - A simultaneous checkout for the last item online and in-store must correctly grant the item to the first requester and gracefully deny the other.
  - The POS UI must function smoothly on a simulated 375px mobile device.
  - The Operations Agent must successfully trigger a notification when an item reaches 0 stock.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
