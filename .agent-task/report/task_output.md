issue_title: "Offline-Tolerant Mobile-First POS & AI Sync Engine"
issue_description: |
  ## Title
  Offline-Tolerant Mobile-First POS & AI Sync Engine

  ## Problem Statement
  For operators like Fatima (Food Cart, 50) and Carlos (Handyman, 42), internet connectivity is not guaranteed. Fatima operates her food cart in busy areas where cellular networks are congested, and Carlos often performs repairs in rural areas or deep inside concrete structures with zero signal. Traditional web-based POS and cloud-first management apps lock up, show infinite spinners, or lose critical data (like a completed service sign-off or pre-order payment) when the network drops. A small business owner cannot wait 2 minutes for an app to reconnect while a customer is standing in front of them. They need an app that feels instant and never loses a transaction, operating primarily out of a local cache that gracefully syncs when a connection is available.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify POS & Square:** Both offer offline modes that queue transactions, but they are heavy companion apps that require separate setup. They do not integrate deeply with an AI assistant that can queue "Draft DMs" or "Agent Workflows" while offline.
  - **Wix & Squarespace:** Essentially useless offline. Their platforms are web-first and rely heavily on constant server connections for state management.
  - **Modern Link-in-Bio tools (Stan, Linktree):** Do not have native offline POS capabilities or robust local data stores.
  - **OHC Opportunity:** By building a true local-first architecture in our Flutter app, we empower owners to use their 375px mobile device as a bulletproof operational tool. When offline, OHC can queue not just data mutations (orders, bookings), but also Agent intents (e.g., "Draft a follow-up email when back online"). This bridges the gap between raw data sync and AI-assisted task queuing.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Mobile Client 375px
          UI[Flutter UI - 375px]
          BL[Bloc / State Management]
          LocalDB[(Local SQLite / Isar DB)]
          SyncQ[Sync Queue & CRDT Logs]
          AgentQ[Agent Intent Queue]

          UI -->|Read/Write| BL
          BL -->|Persist| LocalDB
          BL -->|Enqueue Mutation| SyncQ
          BL -->|Enqueue AI Task| AgentQ
      end

      subgraph Cloud Backend
          API[gRPC / REST Gateway]
          SyncService[Conflict Resolution Engine]
          Postgres[(Tenant-Isolated PostgreSQL)]
          JobQ[AI Job Queue - SKIP LOCKED]
          Agents[Agent Swarm]

          API --> SyncService
          SyncService -->|Write| Postgres
          API --> JobQ
          JobQ --> Agents
      end

      SyncQ -.->|Background Sync| API
      AgentQ -.->|Dispatch when Online| API
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Offline Indicator:** A subtle, unobtrusive amber dot or banner in the top navigation bar indicating "Working Offline - Changes Saved locally."
  - **Checkout Flow:** When creating an order or booking, the flow remains identical. No blocking loaders. The "Complete" button transitions instantly to a success state.
  - **Data Vis:** Lists (like orders or tasks) show a small sync icon next to items that are queued for upload.
  - **Conflict Resolution UI:** If a booking was modified offline but also changed by a customer online, present a clear 375px card: "Booking Conflict: Keep your changes or customer's?"

  ### AI Agent Integration Points
  - **Deferred Agent Tasks:** If Maya tries to reply to an Instagram DM while offline, the text is saved to the `Agent Intent Queue`. The Ambassador agent picks this up and dispatches it immediately upon reconnection.
  - **Offline Summarization:** The Decision Assistant can use locally cached sales data to generate the "Daily Summary" even if the owner is on a subway without signal at the end of the day.

  ### Key Design Decisions
  - **Local-First Reads/Writes:** The Flutter app always reads from and writes to the local database first. Network requests are treated as background synchronization, not blocking operations.
  - **Queue Segregation:** Separate the raw data sync queue (for database mutations) from the AI Intent Queue (for complex, asynchronous agent workflows).

  ## Implementation Prompt
  **User-Facing Outcome:** A non-technical owner like Fatima or Carlos should be able to open OHC, create a new order or complete a service task, and hit "Save" seamlessly even in airplane mode. The app should instantly reflect the change and show a subtle indicator that it will sync soon.

  **CUJ (Critical User Journey):**
  1. Open app (with network disabled).
  2. Navigate to "Orders" or "Tasks".
  3. Create a new entry and tap Save.
  4. See instant success state and offline indicator.
  5. Enable network.
  6. Observe the offline indicator disappear and the item seamlessly sync to the server without further intervention.

  **Acceptance Criteria:**
  - Build a local data repository implementation in Flutter that wraps existing state.
  - Implement a background sync worker that flushes pending mutations when network connectivity is restored.
  - Design and implement the 375px "Offline Mode" visual indicator and state management.
  - Implement at least one E2E Playwright/Flutter test simulating offline mode -> mutation -> online mode -> sync completion.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []