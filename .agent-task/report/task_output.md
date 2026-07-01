issue_title: "Research: Unified Offline-Tolerant Mobile-First POS Sync & Redlock Inventory Coordination"
issue_description: |
  # Research Report: Unified Offline-Tolerant Mobile-First POS Sync & Redlock Inventory Coordination

  ## Problem Statement
  Business owners (e.g., Priya the boutique owner) operating across digital storefronts and physical point-of-sale (POS) systems experience frequent inventory desynchronization. Double-booking occurs when an online purchase goes through just as a walk-in customer is paying for the final item via the POS. Additionally, mobile POS devices often encounter intermittent connectivity on the shop floor or at pop-up locations, causing failed syncs and lost revenue tracking.

  ## Research Report & Gap Analysis
  - **Market Context**: Platforms like Shopify manage multi-channel inventory reasonably well for large merchants, but fail micro-SMEs due to complex app dependencies. Stripe Terminal offers excellent in-person payment hardware, but expects the platform to handle inventory consistency. Link-in-bio tools (Linktree, Stan Store) lack physical POS capabilities entirely.
  - **The OHC Opportunity**: OHC can eliminate "double-selling" by natively integrating Stripe Terminal with our central PostgreSQL ledger using a temporary distributed lock (Redis Redlock). Furthermore, OHC can ensure POS transactions complete smoothly even during network hiccups via an asynchronous, eventual-consistency sync queue.
  - **The Gap**: Currently, OHC lacks a robust, transactional offline-sync mechanism for POS transactions that inherently respects real-time inventory locks. When a mobile POS loses connection, transactions either block entirely or sync later without guaranteeing inventory was reserved, causing painful downstream conflicts.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant OwnerPOS(Mobile)
      participant OfflineSyncWorker
      participant Backend(pos_api.rs)
      participant DistributedLock(Redis)
      participant CentralLedger(PostgreSQL)
      participant Agent(Operations)

      OwnerPOS(Mobile)->>Backend: Initiate POS Checkout (Item A)
      alt Network Available
          Backend->>DistributedLock: Acquire Redlock (Item A, 15s)
          DistributedLock-->>Backend: Lock Acquired
          Backend-->>OwnerPOS(Mobile): Proceed to Payment (Stripe Terminal)
          OwnerPOS(Mobile)->>Backend: Payment Success
          Backend->>CentralLedger: Deduct Inventory
          Backend->>DistributedLock: Release Lock
      else Network Unavailable
          OwnerPOS(Mobile)->>OwnerPOS(Mobile): Queue Offline Transaction (Cash/Offline-auth)
          OwnerPOS(Mobile)->>OwnerPOS(Mobile): Optimistic Local Inventory Deduction
          Note over OwnerPOS(Mobile), OfflineSyncWorker: Connection Restored
          OfflineSyncWorker->>Backend: Sync Offline Transaction (tenant_id, item_id, qty)
          Backend->>CentralLedger: Verify Current Inventory
          alt Inventory Sufficient
              Backend->>CentralLedger: Deduct Inventory & Record Sale
              Backend-->>OfflineSyncWorker: Sync Success
          else Inventory Insufficient (Sold Online)
              Backend-->>OfflineSyncWorker: Sync Conflict (Out of Stock)
              Backend->>Agent: Trigger Conflict Resolution Protocol
              Agent->>OwnerPOS(Mobile): Action Card: "POS Sync Conflict - Manual Review Required"
          end
      end
  ```

  ### Mobile UX Flow (375px)
  1. **POS Interface**: A streamlined, touch-friendly catalog view (≥ 44x44px touch targets).
  2. **Online State**: Tapping an item instantly reserves it (via Redis). The UI moves seamlessly to the payment terminal flow.
  3. **Offline State**: A subtle banner indicates "Offline Mode - Cash Only". Tapping an item queues it locally.
  4. **Sync Resolution**: Upon reconnection, background sync occurs. If a conflict arises (the item was sold online while the POS was offline), the owner receives a high-priority Action Card in their Agent Feed: "Sync Conflict: Item X was sold offline but is out of stock. Review options."

  ### Key Design Decisions
  - **Central Ledger (PostgreSQL)**: Source of truth for inventory counts with row-level locking for critical updates.
  - **Distributed Locks (Redis Redlock)**: Used exclusively during active, online checkout sessions to prevent race conditions between web carts and POS.
  - **Local-First POS Client**: The mobile client caches catalog data locally (IndexedDB) and utilizes an optimistic UI for offline transactions.
  - **Agentic Conflict Resolution**: We shift the burden of resolving complex inventory sync conflicts from the user's manual review to the Operations Agent, which proposes a resolution via the Unified Agent Feed.

  ## Implementation Prompt
  **Feature Name**: OHC Unified Offline-Tolerant POS Sync & Redlock

  **Target Persona**: Priya the Boutique Owner

  **Outcome**: Priya can process an in-store sale that instantly reserves stock, preventing double-bookings with online shoppers. If the store's Wi-Fi drops, she can continue processing cash sales, which the system automatically syncs and reconciles (using the Operations Agent for conflicts) when the connection returns.

  **Critical User Journey (CUJ)**:
  1. Priya logs into the OHC POS mobile view.
  2. She selects the last "Red Dress" for a walk-in customer. (Backend acquires Redis Redlock).
  3. An online shopper simultaneously tries to add the "Red Dress" to their cart but receives a "Currently in another cart" message.
  4. Priya's Wi-Fi drops. She processes the sale as cash. The transaction is queued locally.
  5. The Wi-Fi returns. The app syncs the sale, permanently deducting the "Red Dress" from the PostgreSQL ledger.

  **Acceptance Criteria**:
  - Implement Redis Redlock in the backend checkout/POS API to reserve inventory for 15 seconds.
  - Implement local transaction queuing (IndexedDB) in the mobile POS frontend.
  - Implement the background sync worker that handles reconciliation with PostgreSQL.
  - Update the Operations Agent to generate an actionable feed card if an offline sync results in a negative inventory balance.
  - Ensure UI components adhere to 375px width constraints and 44x44px touch targets.
  - Full E2E Playwright test simulating simultaneous online/offline checkout and conflict resolution.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
