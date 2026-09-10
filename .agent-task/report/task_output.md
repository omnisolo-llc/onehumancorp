issue_title: "Implement Offline-First POS & Mobile Inventory Sync for True Omnichannel Retail"
issue_description: |
  # Problem Statement
  OmniSolo users running physical storefronts or hybrid pop-ups (like Priya the boutique owner or Fatima the food cart operator) face a disjointed experience when taking in-person payments. Existing POS tools like Square or Shopify POS require separate hardware, charge high processing fees for mobile usage, and often drop connection leading to missed sales during rushes or out-of-stock disasters because offline transactions don't immediately sync with the online inventory. We need a unified POS Terminal system that leverages Tap-to-Pay directly on their phone (Android/iOS) seamlessly connected to their single OmniSolo workspace. It must work flawlessly offline, queueing transactions and automatically reconciling inventory without double-selling, and never requires developer configurations.

  # Research Report
  - **Market Context**: Square currently dominates the mobile-first POS segment but traps users in its ecosystem, struggling with deep omnichannel inventory outside its native site builder. Shopify POS is highly robust but requires significant setup, hardware add-ons for tap-to-pay natively, and its offline mode often results in overselling if not carefully managed.
  - **Operator Pain Points**:
    - **Hardware lock-in**: Purchasing dedicated readers (Stripe Terminal/Square Reader) instead of using the smartphone already in their pocket.
    - **Offline unreliability**: Rushes at farmers' markets often have terrible cell service. The system must process (or at least queue) cash and card-on-file/NFC transactions locally and sync idempotently later.
    - **Inventory collisions**: Selling an item offline while someone buys the last unit online.
  - **High-Signal Solutions**: Modern mobile hardware supports Tap-to-Pay natively via Stripe Terminal SDKs on iOS and Android. CRDT (Conflict-free Replicated Data Types) and background event queuing are the standard for robust offline-first sync mechanisms.

  # Design Doc

  ## Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner Phone (PWA/Flutter)
      participant Stripe SDK (NFC)
      participant Local SQLite + Queue
      participant OHC Backend (pos.rs)
      participant OHC Database (PostgreSQL)

      Owner Phone (PWA/Flutter)->>Stripe SDK (NFC): Tap-to-Pay (Offline/Online)
      Stripe SDK (NFC)-->>Owner Phone (PWA/Flutter): Auth/Capture token
      Owner Phone (PWA/Flutter)->>Local SQLite + Queue: Save Transaction & Inventory Delta
      Note over Owner Phone (PWA/Flutter),Local SQLite + Queue: UI instantly updates (no spinner)
      loop Background Sync
          Local SQLite + Queue->>OHC Backend (pos.rs): POST /api/v1/pos/sync (Idempotent UUID)
          OHC Backend (pos.rs)->>OHC Database (PostgreSQL): UPSERT Transactions, Update `inventory_levels`
          OHC Database (PostgreSQL)-->>OHC Backend (pos.rs): Success + Updated Stock
          OHC Backend (pos.rs)-->>Local SQLite + Queue: ACK (Remove from Queue)
      end
  ```

  ## Mobile UX Flow (375px First)
  1. **POS Home (`/pos`)**: A clean grid of core products with large touch targets (min 44x44px). A prominent "Cart" banner at the bottom showing item count and total. macOS translucent glass materials for modals.
  2. **Checkout Modal**: Slides up swiftly. Displays big, friendly buttons for "Tap to Pay", "Cash", and "Send Invoice".
  3. **Offline Indicator**: A subtle, elegant dot indicator (green for online, amber for offline queuing) in the top-right corner. It should not block the user from transacting.
  4. **Post-Transaction**: Instant success haptic feedback and a green checkmark. Automatically returns to the POS Home for the next customer in line.

  ## AI Agent Integration Points
  - **Accounting & Tax Agent**: Automatically categorizes the synced POS transactions into the general ledger and updates daily sales tax liabilities based on the geolocation of the sale.
  - **HR & Logistics Agent**: Monitors the `inventory_levels` mutations from the POS sync. If `available_count` drops below the threshold, it automatically drafts a Purchase Order for the supplier.
  - **Support & Voice Agent**: If a customer email is captured during POS checkout, the agent can send a personalized thank you and review request an hour later.

  # Implementation Prompt
  Implement the backend sync engine for the offline-first OmniSolo POS. Update `src/server/api/pos.rs` to include a `/sync` endpoint that accepts batches of offline POS transactions and inventory deltas. Ensure idempotent processing using a `client_mutation_id`. The endpoint should update `orders`, `inventory_transactions`, and `inventory_levels` while respecting tenant boundaries. Integrate this with the existing caching layer to invalidate edge caches appropriately. Do not implement the Stripe mobile SDKs; focus entirely on the robust backend data reconciliation and API layer for the PWA/Flutter client to consume. Ensure 100% unit test coverage for the new endpoint, verifying idempotency and correct inventory arithmetic.

  # Priority
  P0

  # Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []