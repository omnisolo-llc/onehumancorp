issue_title: "Implement Offline-Tolerant Mobile Point-of-Sale (POS) & AI Local Sync"
issue_description: |
  ## Title
  Implement Offline-Tolerant Mobile Point-of-Sale (POS) & AI Local Sync

  ## Problem Statement
  For operators like Fatima (food cart) and Carlos (field service), internet connectivity is notoriously unreliable. When a food cart operator loses their 4G connection in a crowded downtown square, or a handyman is working in a client's basement, they still need to view the day's tasks, accept cash or pre-authorized orders, and update order statuses. Legacy platforms (Shopify, Wix) often rely on constant web connectivity, causing apps to freeze, show endless spinners, or crash when the network drops. Owners need a mobile command center that is fully functional offline and seamlessly syncs back to the cloud (and the AI agents) when connectivity is restored, ensuring business never stops because of a bad signal.

  ## Research Report
  - **Shopify POS**: Offers an offline mode for cash transactions and basic catalog browsing, but it requires specific enterprise tiers and is largely a distinct app from the main store management interface.
  - **Wix & Squarespace**: Both suffer from poor offline capabilities for their mobile management tools. Trying to fulfill an order offline usually results in network errors.
  - **Square**: The industry leader in offline payments. Square allows taking offline swiped payments and cash, queueing them securely on the device until connection is re-established.
  - **OHC Opportunity**: OHC can differentiate by offering "Local-First AI Operations." Using a local SQLite cache on the device (Flutter/PWA), the app can allow the owner to update task states, add customer notes, or mark orders fulfilled while completely offline. Once online, a sync engine reconciles changes with the multi-tenant PostgreSQL backend and triggers the relevant AI Agent events (e.g., triggering a follow-up email when a task is marked complete).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Mobile UI (375px)
      participant LocalDB as Local Device Cache
      participant Sync as Sync Engine (Background)
      participant API as OHC Backend (Rust)
      participant AI as OHC AI Agents

      Owner->>LocalDB: Mark Order as "Fulfilled" (Offline)
      LocalDB-->>Owner: State updated instantly (No Spinner)
      Note over Owner, Sync: ...Network Restored...
      Sync->>LocalDB: Read pending changes
      Sync->>API: POST /api/v1/sync (Pending Changes)
      API->>LocalDB: Confirm Sync & Update local state
      API->>AI: Trigger "OrderFulfilled" Event
      AI-->>API: Draft Customer Review Request
  ```

  ### UI Wireframes & Screen Flow (375px First)
  1. **Feed Screen**: A clean chronological timeline of today's work.
     - Top Bar: A subtle "Offline Mode" translucent banner (orange tint).
     - Cards: OHC Premium Token styling (translucent glass). Large tap targets (min 44x44px).
  2. **Action State**: When an owner taps "Complete Order," the card instantly animates to a completed state using local data. No loading spinners are shown for critical offline-capable actions.
  3. **Re-connection**: The offline banner transitions to a green "Syncing..." and then fades out smoothly, indicating all tasks have reached the cloud.

  ### Mobile UX Flow
  - **Start**: App is open, network drops.
  - **Action**: User taps to complete a task. UI responds instantly with a satisfying micro-interaction. Data is queued locally.
  - **Resolution**: Background task detects network, pushes queue with idempotency keys. Any conflicts are flagged to the Operations Assistant AI, which creates an "Action Needed" card for the owner if manual intervention is required.

  ### AI Agent Integration Points
  - **Event Triggering**: The sync engine must emit domain events (e.g., `OrderFulfilled`, `CustomerNoteAdded`) to the Kafka/Redis queue after syncing.
  - **Conflict Resolution**: If the sync engine detects a conflict (e.g., the owner cancelled an order offline, but a customer paid for it online simultaneously), the AI Operations Agent intercepts the conflict and surfaces a simple resolution card on the owner's feed instead of throwing a raw technical error.

  ### Key Design Decisions and Why
  - **Local-First Writes**: We prioritize immediate UI feedback over strict consistency to ensure field workers (like Carlos) and fast-paced environments (like Fatima's cart) are never blocked by a slow network.
  - **Idempotent Sync**: All sync API calls must use unique idempotency keys generated on the client to prevent duplicate actions if a network connection drops mid-request.
  - **AI-Managed Conflicts**: Instead of asking the user to resolve JSON diffs, AI summarizes the conflict into a simple business decision (e.g., "This order was cancelled locally, but the customer just paid. Issue refund or keep order?").

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your objective is to implement the offline-tolerant task update mechanism for the Mobile-First Operations Assistant.
  1. **CUJ**: A user can mark an `AgentFeedTask` or `Order` as complete while the application has no network connection. The UI must instantly reflect the completed state. When the network is restored, the local state must seamlessly sync to the server without duplicate submissions.
  2. **Acceptance Criteria**:
     - The mobile UI (Next.js/PWA or Tauri) must implement a local store (e.g., IndexedDB/local storage) to queue mutation requests when offline.
     - The backend must expose a sync/batch endpoint that accepts these queued mutations.
     - All mutations must be idempotent.
     - Ensure the design follows the OHC Premium Token translucent glass aesthetic and operates perfectly on a 375px viewport.
     - Add Playwright E2E tests simulating an offline state (using Playwright's offline mode capability), performing an action, restoring the network, and verifying successful sync.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
