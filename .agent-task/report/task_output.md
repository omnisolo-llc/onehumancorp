issue_title: "Offline-First Multi-Tenant Edge Sync Architecture"
issue_description: |
  # Problem Statement
  For small business owners operating in diverse environments—like Fatima running a food cart with spotty connectivity or Carlos working in a customer's basement with no cell service—the inability to reliably access their business platform is a critical failure. They need to view catalogs, log sales, accept deferred payments, and manage inventory without interruption. Currently, OneHumanCorp relies on a continuous connection to the cloud backend. The lack of a robust, offline-first multi-tenant edge sync architecture means our users face severe friction and lost revenue when their connectivity drops.

  # Research Report
  ## Findings & Ecosystem Analysis
  - **Codebase & Docs Audit**: Reviewing `src/` and `docs/` reveals a reliance on standard cloud-centric PostgreSQL and Valkey data flows. While multi-tenancy is respected at the backend API level, there is no standardized framework for local-first sync and conflict resolution on the client side (especially for the Tauri desktop/mobile runtime).
  - **Competitor Systems Audit**: Systems like Square POS and modern offline-first apps (e.g., Linear) heavily rely on local data stores with background eventual-consistency sync mechanisms (like CRDTs or delta-syncing). Shopify POS also offers robust offline modes for essential operations. OHC lacks this foundational capability.
  - **Identify Gaps**: OHC requires a structural capability to cache multi-tenant data securely on the edge (device), allow read/write operations offline, and seamlessly sync state when connectivity is restored, ensuring complete isolation and conflict resolution without manual user intervention.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile UI - 375px] -->|Reads/Writes| B[(Local Edge DB - SQLite)]
      B --> C[Edge Sync Manager]
      C <-->|Offline: Queues Mutations| C
      C <-->|Online: Syncs Deltas| D[API Gateway]
      D --> E[Conflict Resolution Engine]
      E --> F[(Cloud Multi-Tenant DB)]
      E --> G[AI Ops Agent]
      G -->|Resolves Complex Conflicts invisibly| E
  ```

  ### UI Wireframes & Screen Flow (375px Mobile First)
  - **Global State Indicator**: A subtle, beautifully designed indicator in the macOS-style translucent glass header showing a "cloud with a slash" icon when offline, ensuring Maya or Carlos knows they can still operate.
  - **Interaction**: Ubiquiti UniFi modular cards continue to be interactive. A "Checkout" button seamlessly completes a transaction, appending it to the local ledger.
  - **Sync Resolution (Hidden)**: The user never sees a "Sync Conflict" modal. Instead, the AI Operations Department silently resolves logical conflicts (e.g., oversold inventory) in the background and sends a plain language summary (e.g., "Adjusted an order since you were offline") only if critical.

  ### Mobile UX Flow
  1. Fatima takes a pre-order in a busy outdoor market where cell service drops.
  2. She taps the catalog item and hits "Checkout" (transact).
  3. The app responds instantly (<50ms) using the local edge database. The UI subtly indicates the order is saved locally.
  4. Connectivity restores 10 minutes later. The Edge Sync Manager pushes the queued mutations.
  5. The AI Operations Agent verifies the transaction against current cloud inventory and confirms the state.

  ### AI Agent Integration Points
  - **Operations Department**: Automatically handles multi-version concurrency control (MVCC) or CRDT conflicts that cannot be mathematically merged. For example, if two devices sell the last item offline, the AI agent can intelligently decide which order to hold or offer a refund/alternative to the customer.

  ### Key Design Decisions
  - **Offline First & Eventual Consistency**: All reads and writes must hit the local store first to meet strict zero-latency targets.
  - **Zero Trust & Security**: Multi-tenant isolation extends to the edge. The local cache must be strictly scoped to the authenticated tenant and encrypted at rest on the device to prevent data leakage.
  - **No Implementation Prescriptions**: The exact sync protocol (e.g., ElectricSQL, PowerSync, or custom CRDTs) and local schema structures are left to the implementation swarm. The focus here is on the architectural boundary and the UX guarantee.

  # Implementation Prompt
  Design and implement the `Offline-First Multi-Tenant Edge Sync Architecture`.
  - Establish a local caching layer (e.g., SQLite) within the mobile/Tauri runtime that enforces tenant isolation.
  - Implement the `Edge Sync Manager` to handle background queuing of mutations when offline and bi-directional delta syncing when online.
  - Integrate an AI-driven conflict resolution mechanism within the backend to automatically handle complex state merges without user intervention.
  - Ensure the 375px UI continues to function with <50ms latency during offline states, utilizing translucent glass styling for any necessary connectivity indicators.
  - **Acceptance Criteria**: A user can go offline, create three new orders, come back online, and observe the orders perfectly sync to the cloud database within 5 seconds of connection restoration, with no blocking errors or manual conflict resolution prompts shown in the UI.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
