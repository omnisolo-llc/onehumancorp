issue_title: "[Platform] Offline-First Tap-to-Pay Omnichannel Inventory Sync Mesh"
issue_description: |
  # Problem Statement
  Small business owners like Priya (boutique owner) often experience friction when selling at pop-up events or using tap-to-pay terminals in areas with poor or no internet connectivity. When they process an in-person transaction offline, their central inventory does not reflect the change until connectivity is restored. This leads to overselling online and confusion. We need an architectural enhancement that supports offline queueing of transactions and eventual reliable mesh synchronization with conflict resolution to ensure that physical and online inventory remain consistent even under network duress.

  # Research Report
  Current implementation (`src/server/api/offline_sync.rs`) handles basic inventory deduction but lacks robust offline-first caching strategies, true eventual consistency handling on the client side, and integration into the broader Teammate Mesh for cross-channel updates. It operates mostly as a simple REST endpoint.
  Competitive systems (like Shopify POS and Square) utilize a local-first embedded database (like SQLite on the client device or specialized WebSQL/IndexedDB wrappers) to securely queue mutations locally while offline, applying them sequentially.
  They also incorporate "Eventual Consistency" patterns where, upon reconnection, a background service synchronizes the queue with the cloud.

  # Design Doc
  We propose upgrading the "Offline-First Tap-to-Pay Omnichannel Inventory Sync Mesh". This involves:
  1. Client-Side Local Ledger: Introduce a secure, tenant-isolated local persistence layer (using something like drift or sqlite via Tauri/Flutter) on the POS application.
  2. Background Sync Engine: A dedicated daemon that monitors network connectivity and asynchronously replays the local mutation queue to the backend.
  3. Conflict Resolution: The `offline_sync_handler` needs to be idempotent and handle timestamp-based conflict resolution. E.g., if an offline transaction is delayed by 3 hours, and in the meantime the item sold out online, the system needs to record a "negative inventory" or "oversell anomaly" event rather than just rejecting the update, alerting the human via an Agentic Advisory report.
  4. Integration with Teammate Mesh: The `TeammateMeshEvent` is currently published, but we should define specific Agent departments (e.g., Operations / Advisory) that listen to `mesh:inventory:anomaly` and proactively notify the owner if an offline sale conflicts with an online order.

  **Architecture Diagram (Mermaid):**
  ```mermaid
  sequenceDiagram
      participant App as Mobile App (Offline)
      participant LocalDB as Local SQLite
      participant SyncEngine as Background Sync
      participant API as API Server (/api/v1/sync/offline)
      participant DB as Cloud DB
      participant Mesh as Teammate Mesh

      App->>LocalDB: Record Transaction (Prod A: -1)
      App-->>App: Optimistic UI Update
      note over App,LocalDB: Device is offline
      note over SyncEngine,API: Device regains connectivity
      SyncEngine->>LocalDB: Fetch Queued Mutations
      SyncEngine->>API: POST /api/v1/sync/offline
      API->>DB: Apply mutation & check for oversell
      API->>Mesh: Publish mesh:inventory:updated (or anomaly)
      Mesh->>AdvisoryAgent: (If Anomaly) Alert Owner
      API-->>SyncEngine: 200 OK
      SyncEngine->>LocalDB: Mark synced
  ```

  # Implementation Prompt
  Implement the enhanced Offline-First Tap-to-Pay Inventory Sync Mesh Backend capabilities.
  1. Refactor `offline_sync_handler` to accept mutation timestamps and idempotency keys (`transaction_id`).
  2. Implement a robust oversell detection logic that, instead of just using `GREATEST(0, ...)` silently, detects if the inventory drops below 0. If it does, publish a `mesh:inventory:anomaly` event to the Teammate Mesh.
  3. Ensure that the database correctly tracks the transaction to prevent double counting if the client retries the same `transaction_id`. You may need to create a `synced_transactions` ledger table.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
