issue_title: "Implement Offline-Tolerant Tap-to-Pay & Sync Engine for Mobile Operators"
issue_description: |
  ### Problem Statement
  Physical business operators like Fatima (Food Cart) and Priya (Boutique) operate in environments with flaky internet connections (e.g., crowded food truck parks, deep inside concrete retail buildings). Currently, standard checkout flows fail entirely when the network drops, blocking revenue. They need a robust, offline-tolerant Tap-to-Pay and local cart system that works perfectly on a 375px mobile screen, queues transactions locally, and syncs automatically when the connection is restored, matching the reliability of Square or Shopify POS.

  ### Research Report
  **Market Mapping:**
  - **Square POS:** The gold standard for offline mode. Queues payments securely and syncs when back online. Risks are clearly communicated to the owner (e.g., declined cards when synced later).
  - **Shopify POS:** Offers offline cash/custom payments, and recently expanded Tap-to-Pay on iPhone/Android. Relies on robust local caching (CoreData/SQLite).
  - **Stripe Terminal SDK:** Provides Tap-to-Pay on iPhone and Android. Has native offline support capabilities for store-and-forward in specific configurations.

  **Gap in OHC:** OHC lacks a unified offline-first architecture for the Flutter frontend and a synchronized resolution engine on the Go backend to handle eventually-consistent payment capture and inventory deduction without race conditions.

  ### Design Doc

  #### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant O as Owner (Flutter Mobile)
      participant C as Local SQLite/Hive (Cache)
      participant T as Stripe Terminal SDK
      participant API as OHC API Layer (Go)
      participant Q as PG Job Queue

      O->>T: Initiate Tap-to-Pay (Offline)
      T-->>O: Payment Authorized Locally
      O->>C: Store Order & Txn (Status: Pending Sync)
      O-->>O: Show Success UI (375px)

      Note over O,API: Network Restored
      O->>API: Sync Pending Orders & Txns
      API->>Q: Enqueue processing (tenant-isolated)
      Q->>API: Execute Stripe Capture
      API-->>O: Update UI Status to Completed
  ```

  #### Mobile UX Flow (375px First)
  1. **Checkout Screen:** Owner adds items to cart. Large, high-contrast totals.
  2. **Payment Intent:** Owner taps "Charge $XX.XX". If offline, a yellow "Offline Mode" banner appears.
  3. **Tap-to-Pay:** Native OS modal appears (Apple/Google Tap to Pay).
  4. **Success Screen:** Displays clear "Payment Saved - Will Sync Later" message with translucent glass styling.
  5. **Pending Queue:** A small badge on the home screen indicates "X items waiting to sync".

  #### AI Agent Integration Points
  - **Finance & Decision Assistant:** When back online, if a queued payment fails (e.g., card declined upon sync), the AI Assistant immediately drafts a polite SMS/Email to the customer explaining the issue and providing a payment link to complete the transaction.

  #### Key Design Decisions
  1. **Local-First Persistence:** Use Flutter's local database (SQLite/Isar) as the source of truth for the UI. The network is treated as an eventual sync target.
  2. **Idempotency Keys:** Every offline transaction generates a UUIDv4 on the client, ensuring the backend never double-charges when processing the sync queue.
  3. **Tenant-Isolation:** Sync API endpoints enforce strict `tenant_id` verification via SPIFFE/SPIRE JWTs.

  ### Implementation Prompt
  **For the Implementer Agent:**
  Implement the offline-tolerant checkout flow in the Flutter frontend and the corresponding sync API in the Go backend.
  - **CUJ:** As an owner (Fatima), I can add items to an order, process a Tap-to-Pay transaction while airplane mode is active, see the order saved locally, and have it automatically sync to the backend when the network is restored.
  - **Frontend:** Build a 375px-optimized Checkout screen using the OHC Premium Token library (translucent materials, clean Ubiquiti-style layouts). Integrate local caching for the cart and a mock Stripe Terminal SDK interface that simulates offline authorization.
  - **Backend:** Create a Go/gRPC sync endpoint that accepts an array of offline transactions. Use PostgreSQL `SKIP LOCKED` job queue to process these safely. Enforce row-level security for `tenant_id`.
  - **Testing:** Provide a full Playwright E2E test covering the offline-to-online transition, ensuring 100% unit test coverage on the Go sync logic.

  ### Priority
  P1

  ### Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
