issue_title: "Architectural Design: Offline-First Agentic Sync Protocol for Field & Mobile Operations"
issue_description: |
  # Architectural Design: Offline-First Agentic Sync Protocol for Field & Mobile Operations

  ## Problem Statement
  OHC's current web and mobile clients rely heavily on continuous network connectivity for processing work triage, payments, and agent coordination. For personas like Carlos (field service in areas with poor reception) and Fatima (food cart with slow mobile data), transient network failures block critical writes (e.g., accepting a booking, updating order status). OHC needs a robust, offline-tolerant data synchronization and mutation architecture that caches operations locally and syncs them seamlessly when connectivity is restored, without disrupting the AI agent's background coordination.

  ## Research Report
  - **Findings**: Field service operators and street vendors frequently experience network degradation. Solutions like Linear's sync engine and Shopify POS offline mode demonstrate that optimistic UI updates backed by an append-only local mutation log can significantly reduce perceived latency and prevent data loss.
  - **Competitor Analysis**:
    - *Shopify POS*: Allows cash transactions and local cart saves while offline. Reconciles inventory once online.
    - *Linear*: The client functions fully offline, capturing mutations locally (IndexedDB/SQLite) and replaying them.
  - **OHC Gap**: OHC currently lacks an explicit mutation queue for offline-resilient operations. Writes that fail due to flaky networks leave the owner in an ambiguous state, breaking the "Owner Clarity" value.

  ## Design Doc
  - **Architecture Diagram (Mermaid.js)**:
    ```mermaid
    sequenceDiagram
      participant UI as Mobile UI (375px)
      participant LocalDB as Local Store (SQLite/IndexedDB)
      participant Sync as Sync Engine
      participant API as OHC API Gateway
      participant AI as AI Job Queue

      UI->>LocalDB: 1. Optimistic Write (e.g. Complete Task)
      LocalDB-->>UI: 2. Immediate State Update
      Sync->>LocalDB: 3. Poll for Pending Mutations
      Sync->>API: 4. Replay Mutations (when online)
      API->>AI: 5. Trigger Agent Workflows
      API-->>Sync: 6. Ack & Reconcile State
      Sync-->>LocalDB: 7. Mark Synced
    ```
  - **Mobile UX Flow (375px)**:
    1. User marks a task as "Complete" or order as "Ready".
    2. The UI instantly updates with a visual "success" state (translucent green checkmark).
    3. A subtle, non-blocking indicator (e.g., a small cloud icon with a dashed outline) appears on the item to indicate it's "syncing" or "queued for sync".
    4. If offline, the queue quietly buffers. Upon reconnection, a snackbar or subtle top-banner (Premium OHC Token) briefly displays "Syncing work...", then fades when complete.
  - **AI Agent Integration Points**: AI Operations Assistant receives batch updates upon reconnection and evaluates them logically (e.g., suppressing duplicate notifications if the task was completed offline 3 hours ago).
  - **Key Design Decisions**:
    - Adopt optimistic UI updates for non-destructive actions (task status, basic messaging).
    - Use an append-only Local Mutation Queue (using CRDTs or simple last-writer-wins with timestamps).
    - Explicit error states only when a conflict cannot be auto-resolved by the AI (e.g. double-booked slot).

  ## Implementation Prompt
  - **User-Facing Outcome**: Carlos can tap "Mark Job Complete" in a basement with zero cell service. The app instantly responds and updates his feed. Five minutes later, when he reaches his truck, the app quietly syncs the completion and the Customer Assistant emails the invoice without Carlos doing anything else.
  - **CUJ**:
    1. Owner opens app and goes offline.
    2. Owner changes the status of a work item.
    3. UI reflects the change immediately.
    4. Owner goes online.
    5. System syncs in the background, updating the central DB and triggering relevant AI agents.
  - **Acceptance Criteria**:
    - Implementation of a local mutation queue.
    - Optimistic UI updates on mobile viewports.
    - Auto-retry mechanism with exponential backoff on network restore.
    - Zero mock data; must use real backend sync paths.
    - 100% unit test and Playwright E2E coverage for the offline-then-online CUJ.

  ## Priority & Scope
  - **Priority**: P1
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
