issue_title: "Research: Offline-First Background Sync Queue for POS Tap-to-Pay"
issue_description: |
  ## Problem Statement
  Small business owners need reliable payment processing regardless of internet connectivity. Maya (baker) often sells at farmer's markets with spotty cell reception. Carlos (handyman) takes payments in clients' basements where Wi-Fi doesn't reach. Fatima (food cart) operates in crowded festival environments where cellular networks are overloaded. Currently, OneHumanCorp (OHC) relies entirely on cloud connectivity for payment processing and inventory syncing. If the network drops, businesses halt. Competitors like Square and Shopify have robust "Offline Mode" capabilities, allowing merchants to swipe or tap cards, queue the transactions locally, and sync them automatically when connectivity is restored. OHC lacks an edge-caching, local-first synchronization architecture to enable uninterrupted Tap-to-Pay and Point-of-Sale (POS) operations.

  ## Research Report
  We investigated the underlying mobile POS architectures from major competitors to understand how they achieve high availability at the edge. The findings show a clear industry shift towards local-first databases with background synchronization queues.

  ### Competitive Analysis
  | Platform | Offline Mode Capable? | Underlying Tech | Key Constraint |
  |---|---|---|---|
  | Square | Yes (Up to 24h) | Local encrypted SQLite + Sync Queue | Assumes risk for declined cards later |
  | Shopify POS | Yes (Partial) | React Native + local state caching | Needs internet to apply some discounts/sync inventory |
  | Stripe Terminal | Yes (Forwarding) | Stripe Terminal SDK | Specific hardware required (BBPOS/Stripe Reader) |
  | Wix POS | Limited | Web-view wrapper | Highly dependent on constant connectivity |
  | **OHC (Target)** | **Yes (Continuous)** | **Local-First Edge DB + Conflict-Free Sync** | **Must remain zero-config for user** |

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User as Mobile App (375px)
      participant EdgeDB as Local Edge DB (SQLite/IndexedDB)
      participant SyncMgr as Background Sync Manager
      participant Cloud as OHC Cloud / Stripe

      User->>EdgeDB: Initiate Tap-to-Pay Transaction
      alt Network Offline
          EdgeDB-->>User: Queue Transaction & Show Pending UI
      else Network Online
          EdgeDB->>Cloud: Process Payment
          Cloud-->>User: Show Success UI
      end

      SyncMgr->>SyncMgr: Detect Network Restored
      SyncMgr->>EdgeDB: Fetch Queued Transactions
      SyncMgr->>Cloud: Batch Sync Transactions (Idempotency Keys)
      Cloud-->>SyncMgr: Confirm Sync
      SyncMgr->>User: Update UI to Success
  ```

  ### UI Wireframes & Screen Flow (375px)
  - **Checkout Screen:** Clean numeric keypad for amount entry.
  - **Offline Indicator:** A subtle, premium glassmorphism pill at the top right indicating "Offline - Saving payments locally" in amber.
  - **Payment Processing:** When tapped, immediately shows a local success checkmark with a "Pending Sync" badge below it, so the user can quickly serve the next customer.
  - **Sync Completion:** Once back online, the badge quietly transitions to a green "Synced" state.

  ### Mobile UX Flow
  1. Network drops.
  2. Owner enters payment amount on phone.
  3. Customer taps card.
  4. App records intent locally, queues for sync, and shows immediate success to not block the line.
  5. Owner continues taking orders.
  6. Network restores. App silently syncs in background.

  ### AI Agent Integration Points
  - **Finance & Payments Agent:** Monitors the delayed transactions, tracks risk for offline payments, and automatically tags late declines for follow-up.
  - **Operations Agent:** Updates inventory optimistically on the edge and resolves conflicts upon syncing.

  ### Key Design Decisions
  - Optimistic UI updates to ensure zero friction for in-person transactions.
  - Mandatory use of idempotency keys to ensure robust delayed processing without double charging.

  ## Implementation Prompt
  **Task for Implementer:** Build the foundational Offline-First Background Sync Queue for the OHC POS module.

  **User Journey (CUJ):**
  1. The user (business owner) is logged into the OHC mobile app.
  2. The user loses internet connectivity (simulated offline mode).
  3. The user initiates a Tap-to-Pay transaction for a $10 item.
  4. The application saves the transaction locally with a clear "Saved for later" UI indicator.
  5. The user regains connectivity.
  6. The background sync engine detects the network and flushes the queue to the server.
  7. The server processes the payment via the payment provider.
  8. The UI updates to show the transaction as fully complete.

  **Acceptance Criteria:**
  - Implement a robust local storage mechanism (e.g., IndexedDB on web/PWA or SQLite on native) to queue transaction intents.
  - Implement an event-driven background sync manager that listens for network status changes.
  - Ensure all synced transactions utilize idempotency keys to prevent double-charging.
  - Build the UI fallback states (amber offline indicators, pending sync badges) adhering to the OHC Glassmorphism standards (375px responsive).
  - Implement a simulated backend endpoint to receive batch offline transaction syncs.
  - DO NOT prescribe exact database schemas or library choices; optimize for resilience.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
