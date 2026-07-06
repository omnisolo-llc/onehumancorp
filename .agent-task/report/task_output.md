issue_title: "Implement Local-First Offline-Tolerant Mobile POS & Tap-to-Pay Architecture"
issue_description: |
  ### Title
  Implement Local-First Offline-Tolerant Mobile POS & Tap-to-Pay Architecture

  ### Problem Statement
  For physical product and service owners like Priya (Boutique Operator) and Fatima (Food Cart Operator), operating a point-of-sale system requires absolute reliability. The current ecosystem (like Shopify POS or basic Stripe readers) often fails during spotty mobile internet connections (e.g., at farmers' markets or crowded events). If the connection drops, owners cannot process sales or sync inventory, leading to lost revenue and manual reconciliation nightmares. They need a system that acts locally on the 375px mobile screen, allows continued order capture and queuing of payments offline, and automatically syncs state and triggers background AI Agents when connectivity is restored.

  ### Research Report
  - **Shopify POS & Square**: Both offer offline modes for cash transactions, but handling card payments or complex inventory changes often stalls. Square relies heavily on local caching but can suffer from sync conflicts.
  - **Wix & GoDaddy**: Primarily focus on online checkout, with their physical POS offerings feeling like bolted-on webviews rather than native offline-first mobile experiences.
  - **Stripe Terminal SDK**: Supports saving payment methods for future charging and provides some offline flexibility if designed correctly with a local CRDT (Conflict-free Replicated Data Type) or robust job queue.
  - **Finding**: OHC must differentiate by building a truly Local-First architecture for mobile. By utilizing PowerSync or an offline-first SQLite + CRDT strategy, the OHC Flutter app can queue POS transactions locally. When back online, it syncs instantly and triggers the AI Operations Assistant to update inventory, and the Finance Assistant to log the revenue, without the owner ever seeing a loading spinner or error.

  ### Design Doc
  **Architecture Overview**
  The proposed architecture introduces a Local-First Sync Engine in the Flutter client and an Event Sourcing model in the backend to ensure zero data loss and Conflict-free merging of inventory/sales data.

  ```mermaid
  erDiagram
      TENANT {
          uuid tenant_id PK
          string name
      }
      LOCAL_TRANSACTION_QUEUE {
          uuid transaction_id PK
          uuid tenant_id FK
          string status "pending_sync, synced, failed"
          json payload
          datetime created_at
      }
      INVENTORY_LEDGER {
          uuid ledger_id PK
          uuid tenant_id FK
          uuid product_id
          int quantity_delta
          uuid transaction_id
      }
      TENANT ||--o{ LOCAL_TRANSACTION_QUEUE : owns
      TENANT ||--o{ INVENTORY_LEDGER : tracks
      LOCAL_TRANSACTION_QUEUE ||--o| INVENTORY_LEDGER : triggers
  ```

  ```mermaid
  sequenceDiagram
      participant User (Mobile App)
      participant Local DB (SQLite/PowerSync)
      participant OHC API (Backend)
      participant AI Ops Agent
      participant Stripe Terminal

      User->>Local DB: Create Order & Tap to Pay
      Local DB->>Stripe Terminal: Initiate Card Read (Offline capable/Tokenized)
      Stripe Terminal-->>Local DB: Payment Token/Auth
      Local DB->>Local DB: Save Transaction (status: pending_sync)
      User-->>User: Instant UI Success (375px optimized)

      Note over Local DB, OHC API: Connection Restored
      Local DB->>OHC API: Sync pending transactions
      OHC API->>OHC API: Verify multi-tenant isolation (tenant_id)
      OHC API->>AI Ops Agent: Trigger Inventory Reconciliation & Receipt Email
      AI Ops Agent-->>User: Notification: "3 Offline sales synced!"
  ```

  **Mobile UX Flow (375px First)**
  1. **Checkout Screen**: Large, legible product catalog. A persistent sticky bottom bar with "Charge $X.XX" (touch target > 44x44px).
  2. **Payment Modal**: Translucent glass overlay. Clean Ubiquiti-style hierarchy.
  3. **Offline Indicator**: A subtle, elegant yellow status dot on the top right indicating "Offline Mode - Safe to transact".
  4. **Success Screen**: Immediate checkmark. The app does not block the user from starting the next transaction.

  **AI Agent Integration Points**
  - **Operations Assistant**: Subscribes to the sync queue. When transactions sync, it updates central inventory and flags if an item goes out of stock.
  - **Finance Assistant**: Aggregates synced offline transactions into the daily summary ("Fatima, your cart did $450 while offline today. Everything is synced!").
  - **Customer Assistant**: Drafts follow-up SMS/Emails for digital receipts once the payload reaches the server.

  **Security & Zero Trust**
  - Strict row-level security (`ENABLE ROW LEVEL SECURITY`) on `LOCAL_TRANSACTION_QUEUE` and `INVENTORY_LEDGER` based on `tenant_id`.
  - SPIFFE/SPIRE identity used for internal service-to-service communication when processing the sync queue.

  ### Implementation Prompt
  **Role:** Implementer Agent
  **Task:** Implement the local-first POS transaction queue and synchronization engine.
  **CUJ (Critical User Journey):**
  1. A user (e.g., Fatima) operates the mobile POS interface.
  2. The network connection drops.
  3. The user adds an item to the cart and clicks "Charge".
  4. The system securely tokenizes the payment or logs a cash transaction in the local SQLite database (`LOCAL_TRANSACTION_QUEUE`) with a `pending_sync` status.
  5. The UI immediately shows a success screen, allowing the next customer to be served.
  6. When the network is restored, the local database syncs with the Go/Bazel backend.
  7. The backend validates the `tenant_id`, persists the transaction, updates the `INVENTORY_LEDGER`, and triggers the AI Operations Assistant.

  **Acceptance Criteria:**
  - 100% unit test coverage for the local queue and sync logic.
  - Playwright E2E test verifying the flow from offline mode (simulated) -> transaction creation -> online mode -> successful backend sync.
  - UI must have no hardcoded mock data and strictly adhere to 375px viewport dimensions with translucent premium design tokens.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
