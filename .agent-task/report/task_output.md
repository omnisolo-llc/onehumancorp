issue_title: "Implement Offline-Tolerant Mobile Point-of-Sale (mPOS) Sync Engine"
issue_description: |
  # Research Report: Offline-Tolerant mPOS & AI Financial Reconciliation

  ## Problem Statement
  Operators like Fatima (Food Cart) and Carlos (Field Service) conduct business in environments with unreliable cellular networks or Wi-Fi (e.g., crowded festivals, rural areas, or concrete basements). Existing cloud-based POS solutions freeze or fail to process transactions when offline, leading to lost sales, frustrated customers, and manual paper tracking. When connectivity is restored, manual reconciliation of offline cash/card sales with online inventory and ledgers is time-consuming and error-prone.

  ## Research Report & Competitive Analysis
  - **Square POS / Stripe Terminal:** Both offer "Offline Mode" that queues card-present transactions. However, this relies entirely on hardware capabilities and requires manual sync verification by the user. If the cache clears or hardware fails before sync, data is lost. They also lack built-in multi-tenant agentic workflows to automatically handle sync conflicts.
  - **Shopify POS:** Offers basic offline functionality for cash sales, but card processing requires connectivity. Inventory sync is delayed, often leading to overselling online while the POS is offline.
  - **OHC Opportunity:** Implement an edge-first data strategy using local SQLite (via PowerSync) synchronized with the central PostgreSQL database. This allows full catalog browsing, order creation, and cash/terminal transaction queuing entirely offline. The key differentiator is the **Operations and Finance AI Agents**, which automatically resolve inventory conflicts and reconcile offline ledgers seamlessly upon reconnection, requiring zero manual intervention.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Flutter Mobile Client - 375px] --> B[Local SQLite Database]
      A --> C[Stripe Terminal SDK]
      B --> D{PowerSync / Sync Engine}
      D --> E[(Central PostgreSQL Ledger)]
      D --> F[Event Mesh: Transaction Synced]
      F --> G[Operations Agent The Manager]
      F --> H[Finance Agent The Accountant]
      G -->|Check Conflicts| I[Inventory Redlock/Resolution]
      H -->|Update Balances| J[Daily Summary Push]
  ```

  ### Mobile UX Flow (375px First)
  - **State Indicator:** A pill at the top of the UI gracefully transitions from green "Online" to yellow "Offline Mode."
  - **Offline Cart:** The user can continue adding items from the cached catalog. The checkout button remains active.
  - **Offline Checkout:** For cash or offline-capable terminal payments, the transaction is marked "Pending Sync." A local receipt is generated.
  - **Sync Resolution:** Upon reconnection, the yellow pill turns green, and a subtle "Syncing X transactions..." toast appears. If an inventory conflict occurs (e.g., item sold out online while POS was offline), the user receives a push notification: "Offline sync complete. 1 item oversold, restock order drafted by Operations Agent."

  ### AI Agent Integration Points
  - **Finance Agent (The Accountant):** Reconciles batch offline transactions against the central ledger, identifying discrepancies or missed syncs, and includes them in the daily financial summary.
  - **Operations Agent (The Manager):** Handles race conditions. If an item sold offline was also sold online, the agent prioritizes the offline (physical) sale, automatically refunds/credits the online customer, and drafts an apology email via the Customer Success Agent.

  ### Key Design Decisions
  - **Local-First Writes:** All POS transactions write to the local SQLite database first, ensuring instant UI feedback regardless of network state.
  - **CRDTs / Event Sourcing:** Sync engine uses logical timestamps to ensure eventual consistency without data loss.
  - **Silent Agent Resolution:** Conflict resolution logic is pushed to background AI agents rather than blocking the user interface with error popups.

  ## Implementation Prompt
  **User-Facing Outcome:** Fatima can continue taking food orders and processing payments even when the festival grounds lose cellular service. The app never freezes. When she drives home and connects to Wi-Fi, the app silently syncs all sales, and she receives a single notification confirming the day's revenue and updated inventory.

  **CUJ & Acceptance Criteria:**
  1. Initialize the app in POS mode and disconnect the network simulator.
  2. The UI must display an "Offline" indicator.
  3. Create an order and process an offline payment; the transaction must be saved to the local SQLite store and UI updated instantly.
  4. Reconnect the network simulator.
  5. The sync engine must automatically detect the connection, push the `PosOfflineTransaction` to the backend `PosService.SyncOfflineTransactions` gRPC endpoint.
  6. The backend must ingest the transaction into PostgreSQL and trigger an event for the Finance Agent.
  7. Provide Playwright E2E tests simulating this exact offline-to-online transition using network throttling/disconnection features.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
