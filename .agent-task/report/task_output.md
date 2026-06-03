issue_title: "[architecture] Unified Omni-Channel POS Queue and Sync Mesh"
issue_description: |
  # Architecture Research Report: Unified Omni-Channel POS Queue and Sync Mesh

  ## Problem Statement
  Small business owners need a reliable, fully functional Point-of-Sale experience regardless of internet connectivity. Currently, OHC heavily relies on cloud-centric processing. When a merchant (e.g. Fatima with her food cart at a crowded festival, or Priya selling clothes at a weekend market) loses connectivity, they can't process transactions effectively or keep their inventory in sync across channels.

  ## Research Findings
  Competitors like Square and Shopify have offline modes, but they often restrict functionality or require proprietary hardware. OHC needs a robust, local-first queue and sync mechanism. The core gap identified is a robust local event store and sync engine in the mobile app (or edge client) that gracefully handles transactions locally when offline and reliably synchronizes them to the cloud backend (and AI agent departments) when connectivity is restored, ensuring idempotency and strictly adhering to multi-tenancy requirements.

  ## Proposed Architecture & Design

  ### Architecture Diagram (Mermaid)
  ```mermaid
  sequenceDiagram
      participant App as Mobile App (Edge Client)
      participant LocalDB as Local Sync Queue (CRDT/IndexedDB/SQLite)
      participant NetMonitor as Network Monitor
      participant SyncWorker as Background Sync Worker
      participant Gateway as OHC API Gateway
      participant DB as Cloud Postgres (Tenant-isolated)
      participant FinanceAgent as Finance AI Agent

      App->>LocalDB: Enqueue Transaction Intent (Offline/Online)
      LocalDB-->>App: Ack (UI updates to 'Saved for Later' or 'Processing')

      loop Sync Loop
          NetMonitor->>SyncWorker: Network Restored Event
          SyncWorker->>LocalDB: Fetch Pending Transactions
          SyncWorker->>Gateway: Batch Upload Transactions (w/ Idempotency Keys)
          Gateway->>DB: Process & Persist (Tenant Scoped)
          DB-->>Gateway: Ack
          Gateway->>FinanceAgent: Trigger Reconciliation & Actions
          Gateway-->>SyncWorker: Sync Success Response
          SyncWorker->>LocalDB: Mark Complete/Remove
      end
  ```

  ### Mobile UX Flow
  - Layout is mobile-first (375px baseline) using OHC's signature glassmorphism.
  - While offline, a subtle amber "Offline" indicator appears. The "Tap to Pay" or "Log Cash Sale" buttons remain fully active.
  - Transactions are saved instantly to the local store (sub-50ms response), displaying a checkmark and "Saved for later" state.
  - When the app detects network restoration, the queued indicator switches to a syncing state, and upon completion, a transient success notification is shown.

  ### AI Agent Integration
  - **Finance Agent**: Automatically monitors the sync stream for offline transactions that failed later (e.g., card declined upon sync) to draft an SMS or email to the customer for alternative payment.
  - **Operations Agent**: Uses the sync stream to update global inventory and prevents double-booking.

  ## Implementation Prompt (For Implementer Agent)
  Implement the foundational "Offline-First Background Sync Queue" mechanism.
  - **Outcome**: A local storage and sync engine that intercepts POS transaction intents. If offline, they queue securely locally; if online or when reconnected, the engine batches and flushes them to a backend sync endpoint.
  - **CUJ**:
    1. User logs in.
    2. Connection drops (simulated offline mode).
    3. User processes a $10 payment.
    4. The app queues it locally and UI reflects "Saved for later".
    5. Connection restores.
    6. Background worker flushes the queue to the server.
    7. UI updates transaction status to complete.
  - **Acceptance Criteria**:
    - Local robust storage implementation (e.g., IndexedDB on web/PWA or SQLite on native) for queueing intents.
    - Event-driven sync manager hooked to network status.
    - Uses unique idempotency keys for every synced transaction.
    - A corresponding backend endpoint that handles the batched sync.
    - Zero mock data in UI components.
    - E2E Playwright test verifying the full offline-to-online cycle.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
