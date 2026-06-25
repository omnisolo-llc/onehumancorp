issue_title: "Design: Offline-Tolerant Mobile-First POS Sync Architecture"
issue_description: |
  ## Title
  Offline-Tolerant Mobile-First POS Sync Architecture

  ## Problem Statement
  Operators like Carlos (field service owner) and Fatima (food cart operator) often work in environments with spotty or non-existent mobile data (e.g., inside customer basements, or at busy street festivals). Currently, OHC requires a continuous network connection to process bookings, orders, and payments. When the network drops, they cannot record a service request, mark an order as picked up, or queue a payment, leading to lost revenue and operational chaos. A non-technical owner shouldn't have to worry about cellular signal to log their work.

  ## Research Report
  - **Market Context**: Square Terminal and Stripe Terminal offer robust offline capabilities, where transactions are queued locally and synced when the connection is restored. However, they are point solutions for payments, not full workflow assistants.
  - **Competitor Analysis**:
    - *Shopify*: Excellent POS app with robust offline caching, but highly complex to configure for service businesses like Carlos'.
    - *GoDaddy / Wix*: Weak offline capabilities; their mobile apps are primarily wrappers around web portals and fail gracefully (or ungracefully) when offline.
  - **The OHC Opportunity**: By implementing an offline-tolerant architecture that leverages local device storage for the assistant queue, OHC can ensure that Carlos and Fatima can continue to take orders and update task statuses offline, with AI agents automatically reconciling the queue once the connection returns.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Mobile UI (375px)
      participant LocalCache as Local SQLite/KV
      participant SyncManager as Background Sync Worker
      participant CoreBackend as OHC Cloud
      participant Ledger as Central DB

      Owner->>LocalCache: Create Order/Task (Offline)
      LocalCache-->>Owner: Immediate Success UI
      SyncManager->>LocalCache: Poll for unsynced items
      alt is Online
          SyncManager->>CoreBackend: Push Sync Payload
          CoreBackend->>Ledger: Apply optimistic updates
          CoreBackend-->>SyncManager: Sync ACK
          SyncManager->>LocalCache: Mark Synced
      else is Offline
          SyncManager->>LocalCache: Retain in queue
      end
  ```

  ### UI Wireframes & Screen Flow (375px first)
  1. **Work Feed Screen (Home)**: Standard list of today's tasks and orders.
  2. **Offline Indicator**: A subtle, translucent glass pill at the top reading "Working Offline - Changes Saved" when network drops. No blocking modals.
  3. **Action Button (e.g., "Complete Task" or "Charge $50")**: Remains fully interactive. Tapping it shows an immediate optimistic success state (e.g., green checkmark).
  4. **Pending Sync State**: An icon (e.g., a cloud with an up arrow) appears next to the completed task in the feed to indicate it is pending sync.

  ### Mobile UX Flow
  - **Action**: Owner taps to create a new order or complete a task.
  - **Immediate Feedback**: The UI updates instantly. The owner feels no friction.
  - **Recovery**: When the phone reconnects to 4G/Wi-Fi, the background sync worker silently pushes the payload. The pending icon disappears.
  - **Conflict**: If a conflict occurs (e.g., inventory mismatch), the AI Work Triage agent creates an urgent notification in the owner's feed to resolve it ("We couldn't sync the order for John. Do you still have 1 cake left?").

  ### AI Agent Integration Points
  - **Reconciliation Agent**: A background agent that monitors the dead-letter queue and sync conflicts. Instead of failing silently or showing a raw database error, it translates the conflict into a plain-language question for the owner's feed.
  - **Customer Assistant**: If a booking is delayed due to offline sync, the Customer Assistant can draft an apologetic text to the customer ("Hi, just confirming we got your order!").

  ### Key Design Decisions
  - **Local-First Writes**: All critical writes (order creation, task completion) must go to the local device storage first, then sync to the cloud. This ensures UI performance and offline tolerance.
  - **Agentic Conflict Resolution**: Instead of building complex conflict resolution UI, we rely on the Work Triage agent to explain the conflict to the owner and offer binary choices.

  ## Implementation Prompt
  **For the Implementer:**
  Your goal is to implement the offline-tolerant sync foundation for the mobile PWA/Flutter app.
  - **User Journey (CUJ)**: The user (e.g., Fatima) opens the app, turns off Wi-Fi/cellular, and takes a new food order. The app must allow the order creation, display it in the list with a "pending sync" indicator, and not show any blocking network errors. When the network is restored, the app must automatically sync the order to the backend, and the indicator should disappear.
  - **Acceptance Criteria**:
    - The "Add Order" flow works without an active network connection.
    - Offline orders are persisted across app restarts (use local storage/SQLite).
    - A background sync mechanism pushes pending orders when the network is available.
    - E2E Playwright test simulating offline mode (using standard Playwright network interception/offline mode) must pass.
  Please design the necessary local storage schema, sync protocol, and API endpoints to support this flow. Ensure the UI adheres to the OHC Premium Token translucent design system.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
