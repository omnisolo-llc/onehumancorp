issue_title: "Implement Offline-Tolerant Optimistic Sync & Local-First Queue Architecture"
issue_description: |
  ## Problem Statement
  For users in environments with spotty or slow mobile internet, such as **Fatima** (Food Cart Operator), the OHC mobile application must remain perfectly functional for critical operational workflows. Currently, when the network drops, attempts to update order statuses or toggle menu item availability either fail outright or hang in a loading state. This breaks the "owner clarity" promise, as Fatima loses trust in what the system has recorded, causing confusion during high-volume periods.

  ## Research Report
  - **Context:** Many small business operators work in physically constrained environments—like food carts, basements, or remote job sites—where consistent 4G/5G is not guaranteed.
  - **Competitive Analysis:** Square Point of Sale provides an "Offline Mode" that queues payments and state changes locally, syncing them automatically when connectivity is restored. Similarly, Linear and Notion heavily utilize local-first architectures (like CRDTs or local event queues) to provide instantaneous UI feedback while reconciling state in the background.
  - **Current OHC State:** OHC heavily relies on synchronous API calls (gRPC/REST) for state mutations. There is a lack of a unified, offline-tolerant mutation queue on the client-side (Flutter/PWA).

  ## Design Doc: Offline-Tolerant Local-First Architecture

  ### Architecture Overview
  The goal is to move from synchronous network mutations to an optimistic, queue-based local-first model for critical operational data.

  1.  **Local Mutation Queue (Client-Side):** All state-changing actions (e.g., "mark order #123 complete", "toggle 'Falafel' sold out") are first appended to a persistent local queue (using IndexedDB on web or SQLite on mobile).
  2.  **Optimistic UI:** The UI immediately reflects the desired state change without waiting for network confirmation.
  3.  **Background Sync Worker:** A background process continuously attempts to flush the local queue to the OHC backend. It handles network failures using exponential backoff.
  4.  **Conflict Resolution & Reconciliation:** The backend processes these queued events. If a conflict occurs (e.g., an item was deleted elsewhere before the toggle arrived), the backend handles it gracefully and the client is eventually synchronized via the standard read path (or push events).

  ### Mermaid Diagram
  ```mermaid
  sequenceDiagram
      participant User as Fatima (Owner)
      participant UI as OHC Mobile App (UI)
      participant Queue as Local Mutation Queue (SQLite/IDB)
      participant Sync as Background Sync Worker
      participant API as OHC API Server
      participant DB as OHC Database

      User->>UI: Toggles "Sold Out" on Falafel
      UI->>Queue: Append mutation: UpdateItem(ID, SoldOut=true)
      UI-->>User: Optimistic UI updates instantly
      Note over UI, Queue: Operation completes locally

      loop Every few seconds or on network restore
          Sync->>Queue: Fetch pending mutations
          Sync->>API: POST /api/v1/sync (Batch Mutations)
          alt Network Success
              API->>DB: Apply mutations
              API-->>Sync: 200 OK
              Sync->>Queue: Remove processed mutations
          else Network Failure
              Note over Sync: Retries with exponential backoff
          end
      end
  ```

  ### Mobile UX Flow (375px)
  1. Fatima views her Daily Order List.
  2. She taps a 44x44px "Complete" button on an order.
  3. The order immediately moves to the "Completed" section (optimistic update).
  4. A subtle, non-blocking status indicator (e.g., a tiny cloud icon with an arrow) appears in the header, indicating background sync is in progress.
  5. Once the network syncs, the icon disappears. If the device remains offline, the app functions normally, and the indicator remains visible but unobtrusive.

  ### AI Agent Integration Points
  - **Operations Assistant:** The backend sync endpoint must coordinate with the Operations Assistant. When offline events finally hit the server, the agent must be aware that these events occurred *in the past*. The events must include client-side timestamps.
  - **Work Triage:** If a sync conflict occurs that the system cannot automatically resolve (e.g., a customer canceled the order while Fatima marked it complete offline), the AI Triage agent generates an "Action Card" for Fatima to review the discrepancy, rather than failing silently.

  ## Implementation Prompt

  **Target Persona:** Fatima the Food Cart Operator.
  **CUJ:** Toggling menu item availability and marking orders complete while temporarily disconnected from the cellular network.

  **Instructions for Implementer:**
  1. Implement a persistent local mutation queue in the Flutter/PWA client.
  2. Refactor the `OrderCard` and `MenuItemToggle` components to use optimistic UI updates, writing their intended changes to the local queue instead of awaiting direct API responses.
  3. Create a background sync worker that flushes the local queue to the backend API. Ensure it uses exponential backoff for network failures.
  4. Implement a subtle, non-intrusive UI indicator in the global layout showing pending offline sync status.
  5. Ensure backend endpoints processing these sync requests accept and respect client-provided timestamps to maintain event ordering.
  6. Verify this behavior thoroughly in Playwright E2E tests by simulating network disconnection (using Playwright's offline mode features), performing UI actions, restoring the network, and verifying the backend state.

  **Acceptance Criteria:**
  - The UI must instantly reflect state changes when tapping buttons, regardless of network status.
  - Mutations made while offline must eventually reach the database once the network is restored.
  - The user must never see an infinite spinner or a raw network error dialog when performing core operational tasks offline.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
