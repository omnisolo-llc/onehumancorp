issue_title: "Offline-Tolerant Mobile POS & AI Order Triage for Low-Connectivity Environments"
issue_description: |
  # Research Report: Offline-Tolerant Mobile POS & AI Order Triage for Low-Connectivity Environments

  ## Problem Statement
  Small business owners operating in field services or mobile food environments (like Fatima the Food Cart Operator or Carlos the Handyman) often face unreliable cellular networks and slow mobile data. They need to manage menus, accept pre-orders, and process transactions seamlessly. When connectivity drops, existing cloud-first POS and work triage systems typically freeze, leading to lost sales, missing notifications for pickups, and severe operational frustration. OHC must offer an offline-tolerant mobile POS and an AI order triage system that queues actions locally and seamlessly reconciles them with the backend when the network recovers.

  ## Research Report
  **Market Context & Findings:**
  - **Square & Toast:** Offer offline mode for taking payments, but their cloud-based menu management and real-time AI analytics often become unavailable offline.
  - **Shopify POS:** Relies heavily on an active internet connection to synchronize inventory and fulfill orders. Offline operations are limited, and AI features like Sidekick are entirely cloud-bound.
  - **Wix POS:** Similarly struggles when the network degrades, preventing the owner from seamlessly managing incoming demand or updating product availability.
  - **OHC Opportunity:** By combining Flutter's local database (SQLite/Isolate) capabilities with the Go backend's synchronization queue, OHC can create a truly offline-first experience. Our AI agents can process simple triage commands on-device, or queue natural language inputs to be processed instantly when the connection returns, ensuring Fatima never loses a customer order due to a dropped connection.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Mobile Device "Flutter App (375px)"
          UI[Mobile UI]
          LocalDB[(SQLite Local Storage)]
          SyncManager[Offline Sync Manager]
          LocalQueue[Action Queue]
          LocalAI[Local Fallback AI Router]
      end

      subgraph Edge / Cloud "Go + Bazel + PostgreSQL Backend"
          API[API Gateway]
          SyncEngine[CRDT Sync Engine]
          DB[(PostgreSQL Tenant DB)]
          AI_Ops[Operations Agent]
          AI_Triage[Triage Agent]
      end

      UI --> LocalDB
      UI --> SyncManager
      SyncManager <--> LocalQueue
      LocalQueue -.->|Network Restored| API
      API --> SyncEngine
      SyncEngine --> DB
      SyncEngine --> AI_Triage
      SyncEngine --> AI_Ops

      LocalAI -.->|Provides basic caching| UI
  ```

  ### Mobile UX Flow (375px First)
  - **Home Dashboard (Offline State):** The app detects a poor connection and subtly transitions the top status bar to "Offline Mode - Queuing Actions".
  - **Order Entry:** Fatima taps "Add Order". The UI remains snappy and instantly records the order to local SQLite.
  - **Availability Toggle:** Fatima toggles an item to "Sold Out". The LocalDB updates immediately, and the action is placed in the `LocalQueue`.
  - **Sync Recovery:** Once connectivity is restored, the `SyncManager` flushes the queue to the Go `SyncEngine`. The Operations Agent reconciles the inventory, and the Triage Agent processes any pending customer notifications.
  - **Visual Design:** Glassmorphism cards with a subtle amber glow indicating pending sync operations, transitioning to green upon successful reconciliation. Touch targets remain robust (44x44px).

  ### AI Agent Integration Points
  - **Local Fallback AI Router:** Intercepts basic queries or tasks while offline, providing cached responses or explicitly stating "I'll handle this as soon as we're back online" to maintain trust.
  - **Operations Agent (Cloud):** Upon sync recovery, evaluates the batch of queued transactions, updates real-time inventory across other channels, and resolves any booking conflicts.
  - **Work Triage Agent (Cloud):** Processes batched incoming messages or online orders that occurred while the device was offline, grouping them into an easily digestible summary for the owner.

  ### Key Design Decisions
  - **Offline-First Storage:** Local SQLite on Flutter ensures immediate state updates (optimistic UI), preventing the owner from being blocked.
  - **Eventual Consistency:** A CRDT-inspired sync engine in the backend resolves conflicts using timestamps and tenant-specific overriding rules.
  - **Graceful Degradation:** The AI capabilities degrade gracefully—while offline, advanced generative replies are paused, but core operational CRUD actions are queued safely.

  ## Implementation Prompt
  **User-Facing Outcome:** As a mobile food cart operator (Fatima), I can continue to accept orders, toggle menu items as "sold out", and review my daily tasks even when my phone loses cellular signal. Once my connection returns, the app quietly syncs everything and my AI assistant updates me on any online orders I missed.
  **CUJ & Acceptance Criteria:**
  1. The Flutter mobile application initializes a local SQLite database and an offline sync queue manager.
  2. The user toggles "Network Offline" in the test harness.
  3. The user processes a mock POS transaction and marks a catalog item as "Sold Out".
  4. The UI instantly updates, displaying an "Offline (Pending Sync)" indicator, and the changes persist locally.
  5. The user toggles "Network Online" in the test harness.
  6. The sync manager pushes the queued actions to the Go backend API.
  7. The backend `SyncEngine` updates the PostgreSQL tenant database and triggers the Operations Agent to finalize the inventory changes.
  8. Provide Playwright E2E tests: A user interacts with the app while network requests are blocked, verifies optimistic UI updates, restores the network, and asserts that the backend data has been accurately updated.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
