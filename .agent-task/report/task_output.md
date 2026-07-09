issue_title: "Architecture: Offline-First Resilient Work Capture & Background Sync"
issue_description: |
  # Research Report: Offline-First Resilient Work Capture & Background Sync Architecture

  ## 1. Problem Statement
  Field operators and mobile merchants (e.g., Carlos the Handyman in a basement, Fatima the Food Cart owner in a crowded area with spotty coverage) experience severe friction when attempting to capture service completion, new leads, or payments on a flaky connection. Traditional web-dependent apps block the UI with loading spinners, causing data loss, missed follow-ups, and owner frustration. OneHumanCorp (OHC) needs a true offline-first architecture that allows the owner to act without delay, trusting the system to sync when connectivity returns.

  ## 2. Research Report
  - **Market Context**: Square's "Offline Mode" is a major selling point for mobile merchants, allowing payment and order capture without a network. ServiceTitan offers offline sync for field technicians. Conversely, standard Shopify or basic Wix setups fail gracefully but still block the user from taking action until reconnected.
  - **The OHC Opportunity**: Implementing a local-first data layer in the Flutter frontend, combined with a background sync queue, empowers owners to move at the speed of their work. The AI Work Assistant can act on cached data and automatically reconcile state once back online.
  - **Competitor Gaps**:
    - *Square*: Excellent for POS, but lacks integrated service-task capture and AI-driven conflict resolution.
    - *ServiceTitan*: Too enterprise/complex for the micro-SME (Carlos/Fatima).
    - *Shopify*: Primarily web-based POS lacking robust offline service/task coordination.

  ## 3. Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[Flutter UI 375px] --> B[Local SQLite / Drift Store]
      A --> C[Action Queue / Sync Manager]
      C -- "Network Available" --> D[OHC Backend API gRPC/REST]
      D --> E[Central Ledger PostgreSQL]
      D --> F[AI Work Assistant Queue]
      F -- "Conflict/Anomaly" --> G[Operations Agent]
      G -- "Resolution/Notification" --> A
  ```

  ### Data Model & Sync Protocol
  - **Local Store (Flutter)**: Uses Drift or Isar for local persistence of `WorkTask`, `Customer`, and `PaymentIntent` entities.
  - **Action Queue**: All mutative actions (create task, complete job, capture offline cash payment) are written to a local `SyncAction` queue with a unique idempotency key.
  - **Reconciliation & AI Conflict Resolution**: When connectivity is restored, the Sync Manager flushes the queue to the backend. If an update conflicts (e.g., a booking was canceled online while completed offline), the **Operations Agent** intercepts the conflict, safely records both states, and alerts the owner with a recommended next step via the Triage feed.

  ### Mobile UX Flow (375px)
  - **Offline Indicator**: A subtle, non-blocking translucent glass pill at the top of the UI: "Offline • Saving to device".
  - **Optimistic UI**: When Carlos taps "Complete Job & Request Payment," the button immediately transitions to a success state. No blocking spinners.
  - **Sync Resolution**: When back online, the indicator changes to "Syncing..." and then disappears. Any conflicts identified by the AI agent appear as a high-priority card in the unified Work Triage feed.

  ## 4. Implementation Prompt
  **Feature Name**: Offline-First Resilient Work Capture & Background Sync
  **User Persona**: Carlos (Handyman) and Fatima (Food Cart Operator)
  **Objective**: Enable the mobile Flutter app to capture work tasks, customer notes, and offline payment records without network connectivity, automatically syncing to the Go/Bazel backend when restored.
  **Acceptance Criteria**:
  1. Implement a local database abstraction in the Flutter app to cache active tasks and customers for a given tenant.
  2. Implement an Action Queue that intercepts all mutations when offline and replays them to the backend API when online.
  3. Ensure idempotency keys are generated on the client for all queued actions.
  4. Design the backend sync endpoint to process bulk queued actions and flag conflicts to the AI Operations Agent queue.
  5. The UI must remain responsive at 375px, showing optimistic updates and a clear offline status indicator without blocking the user.
  6. E2E Test: Simulate an offline state in Playwright/Flutter testing, create a task, restore network, and verify the backend PostgreSQL database reflects the task.

  ## 5. Priority & Scope
  **Priority**: P1 (High)
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
