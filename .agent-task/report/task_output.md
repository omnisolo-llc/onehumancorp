issue_title: "[Platform] Offline-First Tap-to-Pay Omnichannel Inventory Sync Mesh"
issue_description: |
  # Architecture Gap: Offline-First Tap-to-Pay Omnichannel Inventory Sync Mesh

  ## Problem Statement
  Priya (Boutique Owner, 35) relies on a combination of online sales and in-person foot traffic. She often participates in local markets, pop-ups, or simply experiences intermittent network connectivity in her brick-and-mortar store. When she uses Tap-to-Pay for an in-person transaction and the network drops, she risks double-selling inventory or losing the transaction record entirely.

  The platform lacks a robust, offline-first mesh that locally queues transactions securely, deducts inventory optimistically on the client side, and gracefully syncs (with conflict resolution) once connectivity is restored.

  ## Research Report
  - Competitor Analysis:
    - **Shopify POS:** Offers an offline mode, but it explicitly states that inventory isn't updated across the store until connectivity resumes. They buffer transactions locally but warn users not to rely on it for extended periods.
    - **Square POS:** The gold standard for offline payments. Queues transactions encrypted on-device. Syncs automatically in the background. Does not guarantee inventory prevention against double-sells across multiple devices during the offline window.
  - Pain Point Addressed: Unreliable internet connection at trade shows or older brick-and-mortar locations preventing sales.

  ## Design Doc

  ### Architecture
  ```mermaid
  graph TD;
      MobileClient[Mobile POS App - Offline] -->|Queue Tx| LocalStore[Local SQLite/IndexedDB]
      MobileClient -->|Optimistic Deduct| LocalInventory[Local Inventory Cache]
      LocalStore -->|Network Restored| SyncAgent[Sync Agent - Background]
      SyncAgent -->|Batch Mutations| Backend[Rust API Backend /api/v1/sync/offline]
      Backend -->|Validate & Deduct| DB[PostgreSQL - Row Level Security]
      Backend -->|Acknowledge| SyncAgent
      SyncAgent -->|Clear Queue| LocalStore
  ```

  ### UX Flow
  1. Priya taps 'Charge $50' on her phone.
  2. The network drops. The app shows a subtle amber "Offline Mode - Queuing Transactions" banner.
  3. The payment is processed securely (via Stripe Terminal offline mode capabilities or queued for later capture).
  4. The inventory count for 'Blue Dress Size M' drops from 3 to 2 immediately on the UI.
  5. Connectivity restores. The amber banner turns green "Synced" and disappears. The transaction is fully captured.

  ### Implementation Strategy (Not prescriptive)
  - Implement a CRDT-like or Last-Write-Wins queue for offline events.
  - Provide an API endpoint (`/api/v1/sync/offline`) that accepts an array of queued mutations.
  - Handle potential conflicts gracefully.

  ## Implementation Prompt
  Implement the "Offline-First Tap-to-Pay Omnichannel Inventory Sync Mesh".
  Focus on the backend endpoint `/api/v1/sync/offline` and necessary data structures to accept queued mutations from a mobile client.
  Ensure mutations contain a transaction ID, product ID, and quantity deducted.
  Return an acknowledgment status.
  Add tests for the API layer to ensure the sync mesh resolves without error.

  Priority: P1
  Estimated Scope: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
