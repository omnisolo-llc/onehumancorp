issue_title: "Architectural Design: Offline-First CRDT Mobile Sync Protocol for Low-Connectivity"
issue_description: |
  # Architecture Design: Offline-First CRDT Mobile Sync Protocol

  ## Problem Statement
  Small business operators like Fatima (Food Cart Operator) and Carlos (Field Service Owner) frequently operate in environments with poor, intermittent, or entirely unavailable network connectivity. A food cart at a busy festival might lose mobile data, and a handyman in a basement will lose cell reception. When this happens, traditional e-commerce and work management platforms fail—they cannot process orders, update tasks, or record payments. Owners need absolute operational continuity. If they record an order or update a booking while offline, the system must seamlessly capture that intent and reconcile it asynchronously with the central server when connectivity returns, without requiring manual intervention, double-entry, or risking data loss.

  ## Research Report
  - **Market Gap:** Legacy systems (Shopify POS, Square) have limited offline modes (e.g., Square offline payments) but primarily queue simple transactions. They do not robustly support complex state updates (like inventory adjustments, task reassignments, or complex booking modifications) in a fully offline-first manner. Emerging AI builders often rely on constant server-side agent processing, making them completely inoperable offline.
  - **Competitive Analysis:**
    - *Square:* Best-in-class for offline payments but restricted to basic checkout. Operations management requires an active connection.
    - *Shopify POS:* Caches catalog data, but inventory sync and customer creation often require network calls to be verified.
    - *Linear/Figma:* Utilize CRDTs (Conflict-free Replicated Data Types) and local-first architectures for seamless offline editing, but these are not applied to SMB operational and commerce workflows.
  - **OHC Opportunity:** By adopting a Local-First architecture using CRDTs for the mobile app (Flutter/PWA), OHC can guarantee sub-50ms interaction times and 100% offline capability. The owner operates on local state, and the background synchronization engine handles merging and conflict resolution autonomously, enabling Fatima to keep taking orders even when her data drops.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App - Flutter/PWA] -->|Reads/Writes| B(Local Database - SQLite/Isar)
      B --> C{CRDT Sync Engine}
      C -->|Offline| D[Local Mutation Queue]
      C -->|Online| E(gRPC Sync Stream)
      E --> F[API Gateway]
      F --> G[Central Sync Worker]
      G -->|Resolves Conflicts| H[(PostgreSQL Central Ledger)]
      G --> I[Agent Event Mesh]
      I --> J[Operations Agent - Alerts & Audits]
  ```

  ### Mobile UX Flow (375px)
  1. **Network Indicator:** A subtle, translucent pill at the top of the screen displays "Offline - Saving Locally" when the connection drops.
  2. **Action Continuity:** Fatima taps an item on her 375px menu to create a pre-order. The UI updates instantly. No loading spinners block the critical path.
  3. **Pending State:** A small sync icon appears next to offline-created records.
  4. **Background Reconnection:** When the network is restored, the sync icon animates briefly and disappears. The CRDT Sync Engine merges the local mutations with the central server.
  5. **Conflict Resolution:** If a severe conflict occurs (e.g., Carlos and his partner edit the same booking offline), the Operations Agent drafts a resolution card for the Agent Feed: "I noticed conflicting updates for Smith's Booking. I kept the most recent time. Tap to review."

  ### AI Agent Integration
  - **Operations Agent:** Monitors the CRDT merge logs. If a logical conflict arises that CRDT rules can't neatly resolve (e.g., inventory oversold offline), the agent proactively adjusts stock levels on related items, notifies the owner via the Agent Feed, and drafts an apology/refund message for the customer.
  - **Decisions Agent:** Analyzes offline periods to identify patterns (e.g., "You frequently lose connection at the Farmers Market. Ensure your offline catalog is fully downloaded before Saturday mornings.").

  ### Key Design Decisions
  - **Local-First:** All UI reads and writes go to the local SQLite database first.
  - **CRDTs for State:** Core operational entities (Tasks, Orders, Inventory Counts) use CRDT structures (like LWW-Element-Set or PN-Counters) to guarantee mathematical eventual consistency without locking the UI.
  - **Zero Trust & Multi-Tenant Isolation:** Sync streams are authenticated via SPIFFE/SPIRE, and all synced mutations strictly enforce row-level tenant isolation in the central PostgreSQL ledger.

  ## Implementation Prompt
  **Target Persona:** Fatima the Food Cart Operator
  **Outcome:** Implement the local-first CRDT synchronization layer between the Flutter mobile client and the Go backend. When Fatima records an order while offline, the system must persist the state locally and automatically synchronize with the central PostgreSQL database upon reconnection, resolving any conflicts without data loss.

  **Acceptance Criteria:**
  - Establish the local state database schema using a CRDT-compatible structure.
  - Develop the background synchronization worker that queues offline mutations and streams them to the backend when online.
  - Implement backend resolution logic that accepts CRDT payloads and updates the PostgreSQL ledger safely.
  - Design the mobile UI to display subtle "offline" and "syncing" states.
  - Do NOT prescribe the exact database tables or Go API endpoints; focus on building the generic sync pipeline and CRDT data structures.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
