issue_title: "Implement Local-First Multi-Device Synchronization Engine"
issue_description: |
  # [architecture] Local-First Multi-Device Synchronization & State Resolution Engine

  ## Title
  Implement Local-First Multi-Device Synchronization & State Resolution Engine

  ## Problem Statement
  Small business owners like **Priya (boutique owner)** and **Carlos (handyman)** often manage their business across multiple devices (e.g., an iPad at the front desk, an iPhone on the go, and an employee's Android device). Currently, OHC's architecture relies on synchronous cloud round-trips for state updates (e.g., inventory decrements, schedule changes). In low-connectivity environments, or when multiple employees are acting concurrently, this causes severe UI locking, data collision, or "out of sync" errors. Priya shouldn't double-sell a dress because her iPad was offline while her phone sold it, and Carlos needs his schedule to update instantly across his team's devices without waiting for a server spinner. We need a fundamental shift to a local-first architecture where the UI reads and writes instantly to a local database, and a deterministic state resolution engine handles background synchronization and conflict resolution across all nodes invisibly.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify POS:** Struggles heavily with concurrent offline changes across multiple registers. Syncing requires active connectivity and often manual refresh.
  - **Linear/Superhuman:** These are the gold standard for "local-first" sync in SaaS. They use CRDTs (Conflict-free Replicated Data Types) or specialized sync protocols to allow instantaneous UI interactions and seamless background syncing.
  - **Wix/Squarespace:** Completely cloud-dependent. A dropped connection means the management dashboard is non-functional.
  - **Opportunity for OHC:** By adopting a local-first architecture (similar to Linear's sync engine, but adapted for multi-tenant mobile commerce), OHC can offer an app experience that is orders of magnitude faster and more resilient than any competitor. The app will work seamlessly in a subway or a crowded festival, instantly reconciling state when connectivity returns.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      MOBILE_CLIENT_A ||--o{ LOCAL_SQLITE_DB : "Instant Read/Write (Optimistic)"
      MOBILE_CLIENT_B ||--o{ LOCAL_SQLITE_DB : "Instant Read/Write (Optimistic)"
      LOCAL_SQLITE_DB ||--|{ SYNC_WORKER : "Batches mutations"
      SYNC_WORKER }|--|| OHC_SYNC_GATEWAY : "WebSocket/gRPC sync stream"
      OHC_SYNC_GATEWAY ||--o{ EVENT_JOURNAL : "Appends immutable events"
      EVENT_JOURNAL ||--|| STATE_RESOLVER : "CRDT / LWW conflict resolution"
      STATE_RESOLVER ||--o{ CORE_POSTGRES_DB : "Persists truth"
      STATE_RESOLVER ||--o{ AI_OPERATIONS_DEPT : "Flags complex anomalies"
  ```

  ### UI Wireframes & Screen Flow (375px first)
  **Merchant View (Priya's Inventory Management):**
  - **Screen:** Inventory list view.
  - **Layout:** Ubiquiti UniFi modular dashboard cards with macOS-style Translucent Glass materials. Clean typography, large touch targets.
  - **Interaction (Concurrent Editing):** Priya updates the price of a "Summer Dress" on her phone while offline. The UI updates instantly—no spinner. Her employee, on the shop's iPad (online), sells the last dress.
  - **Resolution UX:** When Priya's phone reconnects, the sync engine resolves the state. The dress shows as sold out, but the new price is preserved for future restocks. The UI smoothly animates the state change. A subtle "Synced just now" indicator appears briefly in the header.

  ### Mobile UX Flow
  1. **Action:** User performs an action (e.g., marks an invoice paid, updates inventory).
  2. **Local Write:** The action is instantly written to the local SQLite database.
  3. **Optimistic UI:** The UI immediately reflects the new state. Zero latency.
  4. **Queue & Sync:** The mutation is queued locally. When network is available, the sync worker streams the mutation log to the OHC Sync Gateway.
  5. **Conflict Resolution:** The core State Resolver uses Last-Write-Wins (LWW) or domain-specific CRDTs to merge changes.
  6. **Broadacst:** The resolved state is pushed down to all other active clients via WebSocket.

  ### AI Agent Integration Points
  - **AI Operations Dept:** While basic conflicts (e.g., concurrent name edits) are handled deterministically, semantic conflicts trigger AI. E.g., if Priya offline-refunds an order that her employee simultaneously marked as "Disputed" online, the AI Operations Agent intercepts the collision, preserves the dispute state, and sends an actionable push notification to Priya explaining the situation in plain language.
  - **AI Sync Optimizer:** An invisible AI agent monitors device connectivity patterns and aggressively pre-fetches data (like upcoming schedule data for Carlos) right before the device historically goes offline (e.g., entering a known dead zone).

  ### Key Design Decisions
  - **Local SQLite Source of Truth:** The UI *only* binds to the local database. It never blocks on a network request.
  - **Event Sourcing / Mutation Logs:** Clients sync a log of *mutations* (events), not just the final state, allowing the server to intelligently merge histories.
  - **Domain-Specific Conflict Resolution:** Use appropriate strategies per entity. E.g., Inventory counts use commutative operations (increments/decrements), while text fields use LWW based on logical timestamps.
  - **Zero-Trust Multi-Tenancy:** Sync streams are authenticated via SPIFFE/SPIRE. A client can only ever pull the mutation log for its specific tenant ID.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Implement the Local-First Multi-Device Synchronization & State Resolution Engine.
  - Architect the client-side local database layer (e.g., using SQLite) that the UI will bind to for zero-latency, optimistic updates.
  - Build the bidirectional sync protocol (WebSocket/gRPC) and local mutation queue to handle background syncing of client actions.
  - Implement the server-side Event Journal and State Resolver. You must define deterministic conflict resolution strategies (LWW, CRDTs) for core entities (Inventory, Orders, Schedule).
  - Ensure strict multi-tenant data isolation within the sync stream.
  - The UI must remain completely unblocked during network partitions, adhering to the Translucent Glass / Unifi design system on a 375px viewport. All complex sync logic must be invisible to the user.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
