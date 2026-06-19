issue_title: "Implement Mobile-First Tap-to-Pay Terminal & Offline-Tolerant POS Sync Architecture"
issue_description: |
  **Problem Statement**:
  Carlos (Handyman) and Priya (Boutique Operator) often work in environments with poor network connectivity (e.g., customer basements, crowded pop-up events). Currently, OHC lacks a unified, offline-tolerant tap-to-pay mechanism and POS synchronization architecture. If the network drops, they cannot process transactions, locking up sales and causing inventory discrepancies. Traditional POS systems like Square provide good hardware but fail to integrate with OHC's centralized, AI-driven work feed, forcing owners to manually reconcile offline sales with their online catalog.

  **Research Report**:
  - **Market Context**: Square and Stripe Terminal are industry leaders in tap-to-pay. Square dominates SMB offline sales because of its robust offline mode, but it does not connect deeply with agentic workflows. Shopify POS is robust but requires an expensive tier and doesn't handle offline operations perfectly without dedicated hardware.
  - **Competitor Analysis**: Stripe's iOS/Android Tap-to-Pay SDKs allow standard phones to act as terminals without extra hardware. This perfectly fits OHC's mobile-first mandate. However, handling offline payments requires securely caching payment intents and syncing them when the network returns, alongside adjusting local inventory caches to prevent overselling when back online.
  - **OHC Pain Points**: Our current system does not gracefully handle 375px mobile tap-to-pay checkout flows that can survive network partitions. Without this, Fatima loses food cart sales when mobile data is slow.

  **Design Doc**:
  - **Architecture Diagram**:
  ```mermaid
  erDiagram
      TENANT ||--o{ POS_SESSION : "has"
      POS_SESSION ||--o{ OFFLINE_TRANSACTION : "records"
      POS_SESSION ||--o{ INVENTORY_CACHE : "holds"
      OFFLINE_TRANSACTION }|--|| PAYMENT_INTENT : "syncs to"
      OFFLINE_TRANSACTION }|--|| CENTRAL_LEDGER : "reconciles"

      TENANT {
          uuid tenant_id PK
          string business_name
      }
      POS_SESSION {
          uuid session_id PK
          uuid tenant_id FK
          datetime started_at
          string device_fingerprint
      }
      OFFLINE_TRANSACTION {
          uuid tx_id PK
          uuid session_id FK
          decimal amount
          string status "pending_sync, synced, failed"
          jsonb payload
      }
  ```
  - **Mobile UX Flow**:
    1. Priya selects "Checkout" on her 375px screen. The app attempts to lock the inventory in Redis.
    2. If the network is unavailable, the app falls back to the `HybridCache` and flags the order as `offline_pending`.
    3. The Stripe Tap-to-Pay SDK activates (native mobile integration), prompting the customer to tap their card on Priya's phone.
    4. The transaction is securely stored in local storage (PWA/Flutter).
    5. An optimistic UI update shows "Payment Successful (Pending Sync)".
    6. Background sync worker automatically pushes `OFFLINE_TRANSACTION` to the `CENTRAL_LEDGER` when connectivity is restored, and the Finance Agent verifies the batch.
  - **AI Agent Integration Points**:
    - *Finance Agent*: Reconciles offline batches and flags any discrepancies (e.g., declined offline payments) for the owner's review.
    - *Operations Agent*: Adjusts online inventory immediately once the sync occurs to prevent double-selling.
  - **Key Design Decisions**:
    - Leverage Stripe Tap-to-Pay SDK on Flutter for a hardware-less experience.
    - Implement Eventual Consistency for inventory during offline modes, utilizing optimistic local counts.
    - Use `tenant_id` consistently across all sync queues to ensure Zero-Trust isolation.

  **Implementation Prompt**:
  As an Implementer agent, your task is to build the backend endpoints and data models to support the offline-tolerant POS sync.
  1. Create a secure API endpoint `/api/pos/sync_offline_transactions` that accepts an array of offline transactions.
  2. Define the `offline_transactions` table with Row-Level Security (RLS) enabled on `tenant_id`.
  3. Implement a Redis-backed queue that processes these transactions and safely updates the `central_ledger` and `inventory`.
  4. Ensure the UI gracefully handles the "Pending Sync" state without blocking the user from initiating a new transaction.
  5. Provide a Flutter/PWA service layer mockup for submitting transactions when `navigator.onLine` becomes true.

  Acceptance Criteria: The endpoint must successfully process a batch of offline transactions, update inventory, and handle network failure simulations. Full unit and Playwright E2E test coverage is required for the sync flow. Ensure all UI components follow the OHC Premium Token library, target 375px mobile screens, and pass the "grandmother test". Zero mock data is allowed in UI code.

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []