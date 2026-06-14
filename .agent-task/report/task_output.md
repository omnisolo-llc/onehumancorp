issue_title: "Implement Offline-First Edge Sync Protocol for Intermittent Networks"
issue_description: |
  ### Title
  Implement Offline-First Edge Sync Protocol for Intermittent Networks

  ### Problem Statement
  Our target personas, like Fatima (Food Cart Operator in high-density areas with congested mobile data) and Carlos (Field Service Owner frequently in dead zones like basements), experience degraded reliability when OHC loses network connectivity. The current architecture expects a persistent connection to the PostgreSQL/Redis backend. When connectivity drops, critical actions (e.g., toggling a menu item to "sold out", capturing a lead's contact info, or saving a drafted quote) fail, resulting in lost data and broken trust. We need an offline-tolerant architecture that securely caches the workspace state locally and queues optimistic writes to sync seamlessly once the network is restored.

  ### Research Report
  **Competitor Analysis**:
  - **Square Point of Sale**: Features a robust "Offline Mode" allowing credit card swiping and cash transaction logging without internet. Payments are queued and processed automatically when reconnected within 24 hours.
  - **Shopify POS**: Provides offline cash transactions and local catalog caching, though lacks full offline credit processing.
  - **Linear / Notion**: Utilize local-first CRDTs (Conflict-Free Replicated Data Types) or local IndexedDB/SQLite caching to provide instant UI updates, syncing diffs via background sync APIs.

  **Gap in OHC**:
  While OHC has a centralized PostgreSQL and Redis Redlock implementation for concurrent actions, it lacks an edge-caching layer for the Flutter frontend and a resilient queue for offline mutations.

  ### Design Doc

  **Mobile UX Flow (375px First)**:
  1. **Network Status Indicator**: A subtle, non-intrusive indicator (e.g., a translucent pill at the top of the UI) showing "Working Offline".
  2. **Optimistic UI Updates**: When Carlos adds a "Replace Pipe" task, the card immediately appears in his list with a small syncing icon, indicating it is saved locally but pending cloud sync.
  3. **Graceful Degradation**: Features strictly requiring real-time external APIs (like LLM draft generation or Stripe Checkout) are disabled or gracefully degraded (e.g., "AI Drafting unavailable offline. Write a manual note to sync later.").

  **AI Agent Integration Points**:
  - **Agent Queue Interception**: If an AI action is requested offline (e.g., generate a proposal), the request is stored in the local mutation queue. When back online, the Operations Assistant processes the backlog and notifies the user via the Work Triage feed.

  **Key Design Decisions**:
  1. **Local Storage**: Use SQLite for structured data caching (tasks, orders, catalog) and IndexedDB for the web/PWA target.
  2. **Mutation Queue**: All offline write operations are serialized into a `LocalMutation` table.
  3. **Idempotency & Conflict Resolution**: Every queued mutation has a strict UUID idempotency key. Upon reconnection, sync uses a "last-write-wins" or "server-authoritative" merge logic depending on the entity, leveraging `updated_at` timestamps.

  **Architecture Diagram**:
  ```mermaid
  sequenceDiagram
      participant User
      participant FlutterApp as FlutterApp (Edge)
      participant LocalDB as LocalDB (SQLite)
      participant SyncEngine
      participant OHCBackend as OHC Backend
      participant Postgres

      User->>FlutterApp: Mark Order #1042 as "Paid"
      FlutterApp->>LocalDB: Update Local State (optimistic)
      FlutterApp->>LocalDB: Append to Mutation Queue
      FlutterApp-->>User: UI reflects "Paid (Syncing)"
      Note over SyncEngine: Network Restored
      SyncEngine->>LocalDB: Read Mutation Queue
      SyncEngine->>OHCBackend: POST /api/v1/sync (Batch Mutations)
      OHCBackend->>Postgres: Apply Changes (SKIP LOCKED / Idempotency check)
      OHCBackend-->>SyncEngine: Sync Success & Latest State
      SyncEngine->>LocalDB: Clear Queue & Update Local State
      FlutterApp-->>User: UI reflects "Paid (Synced)"
  ```

  ### Implementation Prompt

  **Objective**: Implement the edge-caching layer and offline mutation queue for the Flutter PWA shell.

  **Critical User Journey (CUJ)**:
  1. User (Carlos) logs in while online. App hydrates the local database with today's jobs.
  2. User turns on "Airplane Mode" (simulating dead zone).
  3. User completes a job and marks it as "Done". The UI immediately updates without error.
  4. User turns off "Airplane Mode". The app detects connectivity, drains the local mutation queue in the background, and successfully updates the central PostgreSQL database.

  **Acceptance Criteria**:
  - A local storage mechanism is implemented to cache active jobs and inventory.
  - A mutation queue service intercepts write actions when offline.
  - The UI reflects pending sync states without blocking the user.
  - Upon reconnection, a background sync worker successfully posts pending mutations to the REST API using idempotency keys.
  - E2E Playwright tests explicitly toggle network emulation to verify the offline-to-online sync flow. Zero mock data is used; actual local DB to backend DB reconciliation must be proven.

  ### Priority
  P1

  ### Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []