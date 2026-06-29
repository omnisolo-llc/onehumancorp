issue_title: "AI-Driven Offline-Tolerant Sync Protocol for Low-Bandwidth Mobile Environments"
issue_description: |
  ## Problem Statement
  For users like Fatima (food cart operator) and Carlos (field service owner), network connectivity is frequently unstable or completely unavailable. Current OHC architecture relies heavily on real-time API calls, which causes the mobile application to stall, lose pending orders, or drop customer communications when the device is offline or on a slow connection. The non-technical owner/operator experiences this as lost revenue and broken trust, as they cannot view their daily schedule or process transactions reliably.

  ## Research Report
  An analysis of competitors such as Square, Toast, and field service management tools (e.g., Jobber) reveals that offline-first data sync is a critical baseline for these personas. OHC currently lacks a standardized way to safely queue, sequence, and reconcile mutations (e.g., order status updates, agent replies) performed offline.
  Our research indicates that an eventual consistency model, combined with an optimistic UI and conflict-free replicated data types (CRDTs) or robust versioned sync (like PowerSync/ElectricSQL), can solve this without exposing technical complexity to the user.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
    MobileClient[Flutter Mobile App - Offline First]
    LocalDB[Local SQLite/Isar Cache]
    SyncEngine[Client Sync Engine - Background Queue]
    PowerSync[PowerSync / Sync Gateway]
    Postgres[(Central PostgreSQL Ledger)]
    AIAgent[Operations AI Agent]

    MobileClient -->|Reads/Writes| LocalDB
    LocalDB -->|Observes| SyncEngine
    SyncEngine <-->|WebSockets/HTTP Sync| PowerSync
    PowerSync <-->|Logical Replication| Postgres
    Postgres <-->|Triggers Events| AIAgent
  ```

  ### Mobile UX Flow
  1. Fatima opens OHC in a low-network zone; the app instantly loads her daily pre-order list from the local cache.
  2. She toggles a menu item to "Sold Out" (optimistic UI update; local database mutation).
  3. The sync engine queues the `UpdateMenuItemStatus` action.
  4. When connectivity is restored, the sync engine pushes the mutation to the backend in the background.
  5. If an online customer placed an order for the item before the sync completed, the Operations AI Agent detects the conflict during reconciliation, auto-refunds the customer, and sends a drafted apology for Fatima to approve.

  ### AI Agent Integration Points
  - **Conflict Resolution:** Operations Agent monitors the PostgreSQL ledger for time-sequence conflicts.
  - **Automated Communication:** Customer Success Agent drafts context-aware messages for users affected by offline-sync conflicts (e.g., double bookings).

  ### Key Design Decisions
  - **Optimistic UI with Local Persistence:** Ensures 375px viewports remain perfectly responsive and block no actions.
  - **Background Sync Queue:** Avoids blocking the main thread; handles exponential backoff invisibly.
  - **AI-Managed Reconciliation:** Shields the owner from "sync conflict" errors, instead presenting them as actionable business events ("Order canceled due to out-of-stock, refund processed").

  ## Implementation Prompt
  **Objective:** Implement the Offline-Tolerant Sync Engine and UI bindings for the OHC Flutter App.
  **CUJ:** Fatima toggles a menu item to 'Sold Out' while her phone is offline. The app immediately reflects the change. Once the network reconnects, the backend is updated, and the Operations Agent gracefully handles any overlapping orders by drafting a customer notification.
  **Acceptance Criteria:**
  - UI mutations must instantly reflect in the mobile app without waiting for network response.
  - Offline mutations must be queued in local storage and replayed upon network restoration.
  - A simulated network failure during an order update must not result in data loss.
  - Do not expose manual "sync" buttons or conflict-resolution technical jargon to the owner.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
