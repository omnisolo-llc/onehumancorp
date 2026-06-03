issue_title: "[Architecture] Offline-Capable Tap-to-Pay POS Engine for In-Person Sales"
issue_description: |
  # Architecture Research: Offline-Capable Tap-to-Pay POS Engine

  ## Problem Statement
  Small business owners like Priya (boutique owner) and Fatima (food cart) rely heavily on in-person sales. Currently, our system lacks robust support for offline-capable, in-person payments. If the internet connection drops or is slow, they cannot process payments, leading to lost sales and poor customer experiences. We need an architecture that supports offline, in-person Tap-to-Pay capabilities that seamlessly sync with the online ledger and inventory systems when connectivity is restored.

  ## Research Report
  Leading POS systems (Square, Shopify POS) offer varying degrees of offline support.
  - **Square** allows offline card processing (store and forward) but requires a connection within 24 hours.
  - **Shopify** relies primarily on online connectivity but allows for cached catalogs and manual recording of cash sales offline.
  - OHC needs to exceed these capabilities by treating offline as a first-class state, using a local-first architecture on the mobile client (Flutter) combined with a robust background sync mechanism to the backend (Go/Bazel).

  **Key Competitor Analysis:**
  - Square: Strong hardware integration, offline mode stores encrypted card data.
  - Shopify POS: Limited offline card processing, good catalog caching.
  - Stripe Terminal: Provides SDKs for iOS/Android with Tap to Pay on iPhone/Android, handling the low-level NFC and security.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  sequenceDiagram
      participant App as Mobile App (Flutter)
      participant SDK as Stripe Terminal SDK
      participant DB as Local Database (SQLite)
      participant API as Backend API (Go)
      participant Queue as Job Queue (PostgreSQL)
      participant Stripe as Stripe API

      rect rgb(240, 248, 255)
      note right of App: OFFLINE MODE
      App->>App: User adds item to cart
      App->>SDK: Process Tap to Pay (Store & Forward)
      SDK-->>App: Payment Queued
      App->>DB: Save Transaction to Pending Sync Queue
      App->>DB: Decrement Local Inventory
      end

      rect rgb(240, 255, 240)
      note right of App: ONLINE RESTORED
      App->>API: POST /v1/pos/sync (Batch Upload)
      API->>Queue: Enqueue Sync Job (SKIP LOCKED)
      Queue->>API: Dequeue Job (Worker)
      API->>Stripe: Finalize Offline Payment
      Stripe-->>API: Payment Confirmed
      API->>API: Resolve Inventory Conflicts
      API-->>App: Sync Complete (Transaction Successful)
      App->>DB: Clear Pending Sync Queue
      end
  ```

  ### Architecture
  We will implement a local-first mobile POS architecture using Stripe Terminal SDK integrated into the Flutter app.

  1. **Mobile Client (Flutter):**
     - Integrates Stripe Terminal SDK (Tap to Pay).
     - Local database (e.g., SQLite/Isar) stores a cached catalog, inventory counts, and pending transaction queue.
     - State management (Riverpod) handles connectivity changes, switching between online and offline modes transparently.
  2. **Backend (Go + Bazel):**
     - Exposes APIs for syncing catalog and inventory to the client.
     - Receives batch transaction uploads when the client comes back online.
     - Resolves inventory conflicts (e.g., sold offline but also sold online).
  3. **Data Flow (Offline Sale):**
     - User selects items -> Client calculates total locally -> User taps card -> Stripe SDK processes (if supported offline) or queues for processing -> Local inventory decremented -> Transaction saved to local queue.
     - Upon reconnection: Background worker pushes queue to backend -> Backend validates and finalizes with Stripe -> Global inventory updated.

  ### Visual Design & Mobile UX
  - **375px First:** The POS screen must be uncluttered. Large hit targets (44x44px minimum) for adding items.
  - **Glassmorphism:** Use subtle blur effects for modals (e.g., the payment confirmation screen) over the main catalog.
  - **Feedback:** Clear, unmistakable visual and haptic feedback when a payment succeeds or is queued for offline sync.
  - **Connectivity Indicator:** A small, unobtrusive status icon showing online/offline state and sync progress.

  ### AI Integration Points
  - **Operations Agent:** Monitors sync status and alerts the owner if transactions have been pending too long.
  - **Finance Agent:** Reconciles offline transactions and identifies any discrepancies once synced.
  - **Advisory Agent:** Might suggest optimal times to sync based on typical network patterns for the user's location.

  ## Implementation Prompt
  **Implementer Agent:**
  Your task is to build the core foundation for the Offline-Capable Tap-to-Pay POS Engine.
  1. Define the backend gRPC/REST API endpoints in Go for the `sync` and `batch_transaction_upload` operations.
  2. Create the corresponding data models and database schema updates (PostgreSQL) to handle pending transactions and resolve inventory conflicts. Ensure row-level tenant isolation is maintained.
  3. Implement the background worker (using the existing `SKIP LOCKED` job queue pattern) to process the batch uploads.
  4. Write comprehensive unit tests for the backend logic.
  5. Provide a clear CUJ (Critical User Journey) and Playwright E2E test verifying a mock offline transaction being synced to the backend.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
