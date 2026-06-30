issue_title: "Architectural Design: Offline-First Synchronized State for OHC Mobile"
issue_description: |
  # Research Report: Zero-Configuration Offline-Tolerant State Synchronization

  ## Problem Statement
  Business operators like Carlos (field service, often in basements without signal) and Fatima (food cart, slow mobile data) rely on OHC to manage their daily work. If the mobile app requires a continuous internet connection to view the agent feed, update a task, or accept an order, the application will feel fragile and slow. Standard blocking API calls lead to infinite spinners, failed operations, and lost data, eroding trust. OHC must provide a seamless, offline-tolerant experience where the owner's actions are instantly reflected in the UI and synced reliably in the background without needing a manual refresh.

  ## Research Report & Competitor Analysis
  - **Square POS & Shopify POS**: Both invest heavily in offline resilience. Square allows offline payments (stored securely and processed when connectivity returns) and caches catalog/inventory data locally to ensure the checkout flow never blocks.
  - **Linear / Notion**: Utilize offline-first architectures with a local database acting as the single source of truth for the UI, applying event-sourcing with backend conflict resolution. This provides zero-latency interactions.
  - **Technology Landscape for OHC (Flutter + Go + Postgres)**: The most resilient and transparent approach is an Outbox pattern for writes combined with a local SQLite cache for reads. This shifts the Flutter app from a "dumb client" to a "local-first" client, providing instant UI updates and background eventual consistency.

  ## Design Doc: OHC Offline-Tolerant Synchronization Architecture

  ### Architecture Design
  1. **Local SQLite Store**: The Flutter app uses a local database (e.g., `drift` or `sqflite`) to store a localized projection of the tenant's data (active tasks, recent agent feed items, customer profiles, catalog).
  2. **The Outbox Queue**: All state-mutating actions (e.g., "approve quote", "mark order complete") are written to a local `Outbox` table alongside optimistic updates to the local read store.
  3. **Background Sync Engine**: A background worker in the Flutter app listens to connectivity changes. When online, it drains the `Outbox` by sending idempotent mutations to the Go backend.
  4. **Backend Conflict Resolution**: The Go backend uses idempotency keys to process updates and returns the canonical state. If conflicts arise (e.g., an agent took action while the user was offline), the server's canonical state overrides the optimistic UI.
  5. **Agent Notification**: If a background AI agent needs to alert the user while offline, it queues the notification in Postgres and dispatches via APNs/FCM which handle device delivery state.

  ```mermaid
  sequenceDiagram
      participant Owner as User (Carlos)
      participant UI as OHC Flutter UI
      participant LocalDB as Local SQLite & Outbox
      participant SyncWorker as Background Sync
      participant API as Go Backend API
      participant DB as Postgres (Tenant)

      Owner->>UI: Taps "Complete Job" (Offline)
      UI->>LocalDB: Update local state & insert to Outbox
      LocalDB-->>UI: Confirm local write
      UI-->>Owner: Instant UI Update (Success)

      Note over SyncWorker: Network becomes available
      SyncWorker->>LocalDB: Read pending Outbox events
      SyncWorker->>API: POST /api/v1/sync (Idempotent)
      API->>DB: Process event & update tenant state
      DB-->>API: Confirm
      API-->>SyncWorker: Sync Success (Canonical State)
      SyncWorker->>LocalDB: Clear Outbox & update local cache
  ```

  ### Mobile UX Flow (375px)
  - **No Constant "Offline" Banner**: Do not show a glaring red "Offline" indicator unless an action is strictly blocked (like Tap-to-Pay). Use a subtle translucent status token at the top of the Agent Feed: "Working Offline - Sync Pending" only if un-synced data exists.
  - **Instant Interactions**: When Carlos taps "Approve" on an agent draft, the card instantly transitions to a "Done" state. No loading spinners.
  - **Pending States**: If an action requires backend validation, it can show a localized pending visual state (e.g., a dashed border or soft opacity) until the sync confirms it.

  ### AI Agent Integration Points
  - **Stale Context Awareness**: When an AI agent drafts a response, it includes a `context_version`. If Maya approves a draft while offline, the background sync includes this version. The backend verifies if the situation changed (e.g., the customer canceled the order) before executing the action.
  - **Local Context Caching**: The app caches core business context locally so that basic generative UI or categorization rules can apply even when disconnected from the backend LLM.

  ## Implementation Prompt
  **Target**: Frontend/Backend Implementers
  **Goal**: Implement the Offline-Tolerant Outbox pattern for the Agent Feed actions.
  **CUJ**:
  1. User logs into OHC and views their Agent Feed.
  2. Network connection is disabled.
  3. User taps "Approve" on a drafted customer response card.
  4. The UI instantly updates the card to "Approved" and logs the action in the local outbox.
  5. Network connection is restored.
  6. The background sync process flushes the approval action to the backend.
  7. The backend processes the action and sends the canonical updated state back to the client.
  **Acceptance Criteria**:
  - The Flutter app uses a local store to cache Agent Feed data.
  - Actions performed offline update the UI instantly without blocking or crashing.
  - A background sync service successfully replays queued actions to the backend using idempotency keys when the network reconnects.
  - Backend idempotent endpoints handle sync requests and resolve conflicts.
  - E2E tests (Playwright/Flutter integration) verify the optimistic UI update and eventual consistency flows.

  **Estimated Scope**: Large
  **Priority**: P0
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
