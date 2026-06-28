issue_title: "Research: Mobile Tap-to-Pay Omnichannel Sync Architecture"
issue_description: |
  # Research: Mobile Tap-to-Pay Omnichannel Sync Architecture

  ## Problem Statement
  Small business owners like Priya (boutique owner) and Fatima (food cart) sell products both in-person and online. Currently, the local terminal sessions (tap-to-pay) and global multi-tenant inventory ledger are disconnected. When a product is sold offline, the online store isn't immediately updated, leading to overselling and inventory confusion. They need a seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item.

  ## Research Report
  Our research into the current small business platforms shows a significant gap:
  - **Shopify POS:** Offers offline capabilities primarily for cash and custom payment types. Card payments require an active connection. Inventory is cached locally, but true optimistic sync for complex catalog updates is limited.
  - **Wix/Squarespace:** Primarily online-dependent. Robust offline-first POS and inventory management are not deeply integrated at the core edge layer, requiring a solid connection for most management tasks.
  - **Square POS:** The industry leader in offline mode, but hardware lock-in is high.

  **OHC Opportunity:**
  OHC can differentiate by natively unifying local tap-to-pay events with the global inventory cache, using background AI agents to resolve state conflicts gracefully in the background without overwhelming the user.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant App as OHC Mobile App (Offline POS)
      participant Spooler as Local Intent Outbox
      participant SyncWorker as Background Sync Coordinator
      participant Backend as OHC Backend (Go)
      participant Redis as Distributed Lock (Redlock)
      participant OperationsAgent as AI Operations Agent

      App->>Spooler: Record Tap-to-Pay Transaction (Optimistic)
      App-->>App: Update Local UI Inventory

      Note over SyncWorker: Network restored

      SyncWorker->>Backend: Submit Transaction Intent (Idempotent UUID)
      Backend->>Redis: Acquire Lock `ohc:lock:{tenant_id}:inventory:{product_id}`
      alt Lock Acquired & Stock Available
          Backend->>Backend: Deduct Inventory Ledger
          Backend-->>SyncWorker: 200 OK (State Synced)
          SyncWorker->>Spooler: Mark Intent Processed
      else Conflict Detected (e.g., Stock Zeroed Online)
          Backend->>OperationsAgent: Trigger Conflict Resolution
          OperationsAgent->>Backend: Resolve (e.g., Refund or Backorder)
          Backend-->>SyncWorker: 200 OK (Resolved State)
          SyncWorker->>App: Update Local Cache & Notify User
      end
  ```

  ### Mobile UX Flow (375px First)
  - **Network Indicator:** A subtle, premium glassmorphism pill at the top of the dashboard. When offline, it gracefully slides down showing "Offline Mode" in a muted amber color.
  - **Action Execution:** User taps "Charge" via tap-to-pay.
  - **Optimistic Feedback:** The UI updates instantly. A small "Pending Sync" icon appears next to the transaction.
  - **Resolution:** When the connection returns, the icon disappears. If there is an inventory conflict, a non-intrusive bottom sheet pops up: "Operations Agent: A conflict occurred during offline sync, order #XYZ was refunded due to online sell-out."

  ### AI Agent Integration Points
  - **Operations Agent:** Intercepts conflicts when an offline tap-to-pay intent fails due to state drift (e.g., trying to sell the last cupcake that was sold online while the app was offline), determines the best fallback, and notifies the user via the Business Advisory department.

  ## Implementation Prompt
  Implement the Core Optimistic Mutation Engine and Outbox Sync Queue for offline tap-to-pay transactions.

  **User Journey (CUJ) & Acceptance Criteria:**
  1.  Define the `OperationIntent` schema locally and on the server.
  2.  Implement a unified `MutationService` in the frontend that applies tap-to-pay checkout state optimistically, saves the intent locally in an outbox queue.
  3.  Build the background sync worker that flushes the outbox sequentially, ensuring idempotency.
  4.  Implement the backend handler to receive `OperationIntent` batches, acquire Redis distributed locks on inventory items, and apply them within a database transaction.
  5.  Trigger a conflict resolution workflow via the Operations Agent if the state timestamp mismatches or stock is depleted.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
