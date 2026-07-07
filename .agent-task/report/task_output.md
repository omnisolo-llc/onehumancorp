issue_title: "Architecture Gap: True Offline-First Data Sync & Conflict Resolution for Intermittent Mobile Usage"
issue_description: |
  ## Problem Statement
  Currently, OneHumanCorp (OHC) components function effectively with a stable network connection, providing a fluid cloud-native work assistant experience. However, a significant gap exists for operators operating in environments with spotty, intermittent, or completely absent network connectivity for extended periods.

  Consider Fatima (food cart operator) or Carlos (field service technician), who frequently operate in locations with low mobile data availability (e.g., crowded festivals, rural areas, or thick-walled buildings). They require a seamless experience for capturing orders, creating estimates, viewing customer history, and completing jobs even when disconnected. A hard failure, a spinner, or an inability to modify data while offline breaks the "OneHumanCorp Promise" of immediate work orientation and frictionless execution.

  The current mobile clients do not have a robust, system-wide, deterministic local-first state synchronization engine. Writes initiated while offline might fail or require manual retry, and conflict resolution upon reconnection is not adequately defined or handled transparently without user intervention.

  ## Research Report
  ### Competitive Landscape & Industry Standard
  - **Linear / Notion / Superhuman**: These platforms heavily utilize Local-First (Offline-First) architectures. Data is mutated against a local embedded database (e.g., SQLite, IndexedDB), which asynchronously syncs to the cloud using CRDTs (Conflict-free Replicated Data Types) or vector clocks. This guarantees zero-latency UI updates and seamless offline capability.
  - **Shopify POS / Square**: Both platforms support an "Offline Mode" which caches essential catalog data and queues local operations (e.g., taking cash or offline-approved card payments), syncing them back to the server once the connection is restored.
  - **The Gap in OHC**: While our backend architecture is incredibly robust (Go/Rust + Bazel, PostgreSQL, Redis, Kubernetes), the client-server interaction model assumes high availability. We need a Local-First caching and mutation sync layer for the Flutter + PWA clients, ensuring that reads and writes are directed to a local store first.

  ## Design Doc
  ### Architecture Diagram

  ```mermaid
  sequenceDiagram
      participant Owner as User (Mobile UI)
      participant LocalDB as Local Store (SQLite/Hive)
      participant SyncEngine as Sync Engine & Queue
      participant API as OHC API Gateway
      participant DB as OHC PostgreSQL DB

      %% Offline / Immediate Interaction
      Owner->>LocalDB: Read essential data (Instant load)
      Owner->>LocalDB: Write Action (e.g., Update Order Status)
      LocalDB-->>Owner: UI Updates Instantly
      LocalDB->>SyncEngine: Add to Sync Queue (Optimistic)

      %% Background Sync when Online
      alt Network Available
          SyncEngine->>API: Push Batch Mutations
          API->>DB: Apply mutations (Resolve conflicts)
          DB-->>API: Confirm success / return merged state
          API-->>SyncEngine: Ack & Sync diffs
          SyncEngine->>LocalDB: Update Local Store
      else Network Unavailable
          SyncEngine-->>SyncEngine: Keep in Queue, retry with backoff
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Offline Indicator**: A subtle, non-intrusive indicator (e.g., a small "Offline" pill at the top of the feed) that doesn't block the UI.
  2. **Optimistic Updates**: When the user performs an action (e.g., moving a task to "Done", saving a quote draft), the UI updates immediately with a local success state. A subtle sync icon or text indicates the change is pending sync.
  3. **Data Availability**: The core "Today's Priorities", cached client profiles, and active product catalogs are fully visible and searchable.
  4. **Conflict Resolution UI**: In rare cases of hard conflicts (e.g., another device modified the same record), an AI Agent surfaces an actionable triage item in the Work Feed: "Conflict on Order #123. You updated it offline, but it was changed by Jun online. Do you want to keep your change or Jun's?"

  ### AI Agent Integration Points
  - **Triage Agent**: Surfacing unresolvable data conflicts to the user in a natural, assistant-like manner.
  - **Operations Agent**: Pre-caching predicted essential data (e.g., next week's bookings, highly active customer records) to the local store proactively.

  ### Key Design Decisions
  - **Local-First Database**: Adopt a robust local database on the client (e.g., SQLite via `sqflite` in Flutter or `drift`).
  - **Mutation Queue**: All mutative actions are pushed to a local queue. The network layer observes connectivity state and flushes the queue asynchronously.
  - **Idempotency & Versioning**: Implement strict resource versioning (e.g., row version or vector clocks) and idempotency keys on the backend to handle duplicated sync attempts safely.

  ## Implementation Prompt
  Implement the foundation for the Offline-First Data Sync engine in the Flutter client.
  1. Define a local database schema that mirrors a subset of core entities (e.g., Tasks, Orders, Customers).
  2. Create a generic Mutation Queue that intercepts API calls, stores them locally with an idempotency key, and immediately updates the local database.
  3. Implement a Background Sync Manager that listens to network connectivity changes and attempts to flush the queue to the backend.
  4. Ensure the UI reads exclusively from the local database (Repository pattern) to guarantee instant loading.
  5. The acceptance criteria include a unit test or integration test proving that a data mutation initiated while the network is simulated as disconnected will succeed locally, queue up, and successfully sync to the backend once the simulated network is restored.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []