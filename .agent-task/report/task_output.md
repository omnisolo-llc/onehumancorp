issue_title: "Offline-First Mobile Operations & Resilient Data Sync"
issue_description: |
  # Research Report: Offline-First Mobile Operations & Resilient Data Sync

  ## 1. Problem Statement
  Service-based small business owners (e.g., Carlos the Handyman, Fatima the Food Cart Operator) frequently operate in environments with poor, intermittent, or non-existent network connectivity (e.g., basements, rural areas, crowded events). Existing cloud-dependent tools fail in these scenarios, preventing them from recording payments, viewing schedules, or updating inventory. This causes operational chaos, lost revenue, and frustration, directly conflicting with the OHC promise of keeping advanced setup hidden and working beautifully on a 375px phone screen under any condition.

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify and Square have offline capabilities for their POS systems, but these are often limited to specific hardware or require manual synchronization steps. Competitors like Wix and general-purpose scheduling tools often become unusable without a connection.
  - **The OHC Opportunity**: By embedding an offline-first architecture directly into the mobile PWA/Flutter app, OHC can provide a seamless experience where the UI never blocks on network requests. Operations made offline are queued and automatically synced when connectivity is restored, without the owner needing to intervene.
  - **Competitor Gaps**:
    - *Square*: Strong offline payments, but scheduling and CRM features often require a connection.
    - *Shopify*: Offline POS is robust, but the core administrative mobile app can be sluggish or unresponsive on slow networks.
    - *Calendly/Acuity*: Completely dependent on real-time cloud sync.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Flutter Mobile App 375px] --> B{Local State Manager / Cache}
      B --> C[Local SQLite/Isar DB]
      B --> D[Offline Mutation Queue]
      D --> E{Network Connectivity Monitor}
      E -- Online --> F[Sync Engine]
      E -- Offline --> D
      F --> G[REST/gRPC API Layer]
      G --> H[Central Ledger PostgreSQL]
      H --> I[AI Event Mesh]
      I --> J[Operations Agent]
  ```

  ### Data Model & Sync Protocol
  - **Local Storage**: Utilize a lightweight local database (e.g., SQLite via `sqflite` or Isar in Flutter) to cache the owner's immediate context (today's schedule, active inventory, recent messages).
  - **Mutation Queue**: All state changes (e.g., marking a job complete, taking a deposit, updating a menu item) are written first to the local database and simultaneously pushed to a persistent local mutation queue.
  - **Optimistic UI**: The UI immediately reflects the local state change. Visual indicators (e.g., a subtle dashed border or a pending icon) show that the action is queued for sync.
  - **Sync Engine**: A background worker monitors connectivity. When online, it flushes the mutation queue to the backend. It handles conflict resolution (e.g., using "last write wins" with server timestamps, or generating an alert for the Operations Agent if a critical conflict occurs).

  ### Mobile UX Flow (375px)
  1. **Action in Offline Mode**: Carlos is in a basement. He opens the app to mark a job "Completed" and logs a cash payment.
  2. **Immediate Feedback**: The UI instantly updates the job status to "Completed" and shows a small, non-intrusive "Pending Sync" icon.
  3. **Background Sync**: Carlos drives to the next job. The app regains connectivity, silently syncs the mutation queue, and changes the icon to a solid checkmark.
  4. **Agent Handoff**: The backend receives the update. The Operations Agent acknowledges the completion and drafts a follow-up review request for the customer.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Offline-First Operations & Resilient Sync
  **Target Persona**: Carlos the Handyman / Fatima the Food Cart Operator
  **Outcome**: The mobile app remains fully functional for core daily tasks (viewing schedule, updating task status, recording offline payments) regardless of network connectivity. Changes sync automatically and optimistically update the UI.

  **Next Actions**:
  1. Implement a local database abstraction in the Flutter app to cache essential daily data (schedule, tasks, basic CRM info).
  2. Create a persistent offline mutation queue that intercepts state-changing actions (e.g., completing a task).
  3. Develop the background Sync Engine to flush the queue to the Go backend when network connectivity is restored, ensuring idempotency on the server.
  4. Design and implement the visual "Pending Sync" state tokens in the OHC Premium Token library for 375px layouts.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
