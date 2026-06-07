issue_title: "Architecture & Implementation: Centralized Inventory & Distributed POS Synchronization"
issue_description: |
  # Mission Queue Protocol Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Currently, OHC lacks a real-time, strongly consistent inventory locking mechanism to handle simultaneous multi-channel sales. When non-technical operators (like Priya, a boutique owner) use our mobile app (POS mode) while an online customer browses their storefront, double-booking and out-of-stock conflicts occur. SMBs face significant revenue and trust loss when an in-person tap-to-pay transaction overlaps with an online cart checkout due to weak consistency between offline, local-first POS caching, and the central PostgreSQL ledger. They require an invisible, automated system that seamlessly syncs operations across all platforms.

  ## Research Report
  Based on competitive landscape analysis (Shopify, Wix, Stripe Terminal):
  *   **Shopify** solves this via robust POS capabilities, but it often confuses micro-SMEs due to the complex integration layer, relying on premium apps.
  *   **Square / Stripe** provide excellent hardware and basic app solutions but lack the integrated *agentic workflow automation* OHC promises.
  *   **The Gap**: OHC requires an end-to-end multi-channel sync protocol utilizing Redis for short-term distributed locking during checkout (online vs tap-to-pay) and an asynchronous queue pattern to update the Central Postgres Ledger. The system must degrade gracefully for offline-first scenarios while keeping the AI agents strictly coordinated.

  ## Design Doc (Architecture & UX Flow)
  ### High-Level Architecture
  The architecture revolves around distributed locking during checkout and asynchronous POS synchronization.

  **Data Models & Sync Layer:**
  *   **Central Ledger (PostgreSQL):** The definitive source of truth for inventory. Uses row-level multi-tenant isolation policies (`ENABLE ROW LEVEL SECURITY`).
  *   **Distributed Locks (Redis Redlock):** Utilizes key pattern `ohc:lock:{tenant_id}:inventory:{product_id}` to prevent race conditions during concurrent checkouts. Online checkouts require an extended TTL (e.g., 300s/5m), while tap-to-pay POS sessions use shorter, aggressive TTLs (e.g., 15s) reflecting rapid real-time interaction.
  *   **Eventual Consistency (Offline POS Sync):** POS clients sync transactions locally, forwarding to `pos_offline_transactions`. A background task handles the sync to the central products ledger.

  ### Mobile UX Flow (Mobile-First 375px)
  *   Priya opens the OHC Mobile App on a 375px viewport. The screen presents a modular, translucent glass "Dashboard" card showing "Today's Priorities".
  *   She enters the **Tap-to-Pay** flow. The POS catalog utilizes large (44x44px minimum) touch targets for quick item selection.
  *   As she taps "Checkout", the UI displays an optimistic "Processing..." loading state while the backend applies a 15-second Redis lock.
  *   If a simultaneous online customer attempts to finalize the purchase for the same item, the online frontend returns a graceful "Item just sold out in-store" state.
  *   Once the POS payment clears (via Stripe Terminal Integration), a "Syncing..." status appears if the network is flaky, eventually reconciling into the central ledger.

  ### AI Agent Integration
  *   **Operations Agent ("The Manager"):** Listens to `PosSyncFailure` and `LowStockAlert` event payloads from PostgreSQL triggers. If stock levels drop below thresholds (e.g., <= 5), the Operations agent queues a notification pushing Priya to restock. If a conflict is unavoidable (e.g., extreme offline mismatch), it drafts an apology and refund proposition for the online customer.

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant App as Mobile App (Priya)
      participant Web as Web Storefront
      participant API as OHC API (Rust)
      participant Redis as Redis (Locks)
      participant DB as PostgreSQL Ledger
      participant OpsAgent as Operations Agent

      App->>API: Initiate Tap-to-Pay Checkout (Product X)
      API->>Redis: Acquire Lock `ohc:lock:tenant:inventory:X` (TTL: 15s)
      Redis-->>API: Lock Acquired
      Web->>API: Initiate Online Checkout (Product X)
      API->>Redis: Attempt Lock `ohc:lock:tenant:inventory:X` (TTL: 300s)
      Redis-->>API: Lock Denied
      API-->>Web: Graceful UI Failure ("Sold Out in-store")

      App->>API: Finalize Payment (Stripe Terminal)
      API->>DB: UPDATE products SET inventory_count = inventory_count - 1
      DB-->>API: New Stock <= 5 (Low Stock Alert Triggered)
      API->>DB: INSERT INTO department_tasks (Operations, LowStockAlert)
      DB-->>OpsAgent: Dequeue Task
      OpsAgent->>App: Push Notification "Product X running low. Draft restock?"
      API-->>App: Checkout Complete
  ```

  ## Implementation Prompt
  **Target Persona**: Priya (Boutique Owner)
  **Critical User Journey**: Priya uses the OHC mobile app for an in-store tap-to-pay checkout. The system must instantly lock the remaining inventory using Redis so that an overlapping online shopper cannot buy the exact same item.

  **Instructions for Implementer Agent:**
  1. **Locking Mechanism**: Validate and harden the Redis Redlock pattern `ohc:lock:{tenant_id}:inventory:{product_id}`. Ensure the API gracefully fails and returns user-friendly messages for the online shopper if an in-store transaction holds the lock.
  2. **Offline-Sync Resiliency**: Harden the `offline_sync` endpoint. Ensure offline mutations (`pos_offline_transactions`) eventually consistency into the main `products` ledger and appropriately handle conflict edge cases.
  3. **Event Notification**: Ensure `LowStockAlert` and `PosSyncFailure` tasks are consistently pushed to the `department_tasks` queue so the Operations Agent can accurately draft notifications or follow-ups.
  4. **Mobile UX Target**: Ensure any UI rendering these states accommodates 375px viewports natively with touch targets ≥ 44px. Do not introduce any UI mocks; all data must hit the real database/Redis stack. Ensure all tests (`bazel test //...`) pass with 100% coverage on new modifications.

  ## Priority & Scope
  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []