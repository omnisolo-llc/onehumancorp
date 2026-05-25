issue_title: "Offline-First Hybrid Data Sync Mesh"
issue_description: |
  # Title
  Offline-First Hybrid Data Sync Mesh (Stand-in to Cloud Resiliency)

  ## Problem Statement
  Fatima runs a busy food cart and frequently operates in areas with spotty 4G/5G connections. Carlos, a handyman, often works in basements with zero cell service. If they lose connection while trying to check inventory, book a time slot, or process a payment, their business comes to a grinding halt. Current web-based platforms (like Shopify or Wix) simply show a "No Internet Connection" dinosaur game. This is unacceptable. They need their OneHumanCorp (OHC) app to function seamlessly even when offline, continuing to log sales, modify inventory locally, and handle draft messages, and then automatically and invisibly sync everything to the cloud the millisecond connection is restored.

  ## Research Report
  *   **Shopify POS**: Has offline mode but it is limited to taking cash payments and requires the expensive POS Pro add-on. Cloud syncing often leads to race conditions if the connection flickers.
  *   **Wix / Squarespace**: Completely cloud-dependent. Apps become read-only or unresponsive during network outages.
  *   **Square**: Supports offline card payments and basic queuing, but lacks native AI agent syncing (i.e. agents can't process background ops during offline periods).
  *   **The OHC Gap**: OHC's "Hybrid Architecture" (OHC-HA) mentions SQLite-backed local instances, but we lack the foundational sync engine that guarantees reliable state reconciliation between the local SQLite replica on the 375px mobile device and the Multi-Tenant Cloud Postgres DB. We need an Offline-First Sync Mesh to deliver the "Zero-Drop" experience.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Mobile Device (375px)
          UI[Translucent Glass UI] -->|Reads/Writes| LocalDB[(Local SQLite SIPDB)]
          LocalDB -.->|Local Event Bus| LocalAgent[Local Fallback AI Agent]
      end

      subgraph OHC Cloud (Multi-Tenant)
          SyncGateway[Hybrid Sync Gateway] --> CloudDB[(Postgres Core Ledger)]
          CloudDB --> CloudAgents[Cloud Swarm Orchestration]
      end

      LocalDB <==>|Background CRDT/Delta Sync| SyncGateway
  ```

  ### UI Wireframes & Screen Flow (375px Mobile-First)
  - **Top Navigation Bar**: Needs an invisible-unless-critical status indicator. When offline, a soft translucent amber dot appears near the profile icon. Tapping it shows a bottom-sheet (macOS-style blur): "Working Offline. Your changes are saved safely on your phone and will sync when you have service."
  - **Dashboard Cards**: Ubiquiti UniFi style layout. Cards continue to load instantly since they read from local SQLite. Actions like "New Sale" or "Update Price" show immediate optimistic UI updates (e.g. green checkmark).
  - **Grandmother Test**: Ensure "Sync" is never a manual button the user has to press. It must be entirely invisible and autonomous.

  ### Mobile UX Flow
  1. Fatima loses network connection while adding a new menu item and processing a $12 cash order.
  2. The UI responds instantly, showing the new item and the order in the local transaction ledger.
  3. The app writes the changes to the local SQLite DB as "Pending Sync".
  4. Once 4G is restored, the OHC-SIP (Swarm Intelligence Protocol) Sync Mesh automatically pushes the changes to the cloud.
  5. The Cloud Operations Agent validates the data and updates the global inventory. No double-counting occurs.

  ### Key Design Decisions
  - **Offline-First by Default**: The mobile app NEVER reads directly from the cloud for critical paths. It always reads from the local SQLite DB. The Sync Mesh handles pulling updates in the background.
  - **CRDT / Delta Sync Strategy**: To prevent conflicts when Maya updates inventory from her iPad at home and her iPhone at the market, the sync engine must resolve state using Conflict-Free Replicated Data Types (CRDTs) or timestamped delta ledgers.
  - **Agent Handoff**: If the Cloud AI is unreachable, a lightweight "Fallback Agent" running locally on the device (if capable) handles basic validations (like ensuring required fields are met for an order).
  - **Strict Multi-Tenant Isolation**: The Sync Gateway must securely identify the edge device using SPIFFE/SPIRE (or equivalent OIDC token mapping) and ensure the payload only merges into the correct tenant's ledger.

  ### AI Agent Integration Points
  - **Operations Agent**: Monitors the Sync Gateway. If an offline queue is uploaded that contains a conflict (e.g., sold an item locally that sold out online 5 minutes ago), the Operations Agent immediately sends a proactive DM to the merchant resolving the conflict via a refund or alternative offer.
  - **Customer Success Agent**: Queues outbound IG DMs locally. Once synced, fires them off via the cloud.

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the Hybrid Data Sync Mesh bridging the mobile app's local SQLite database and the cloud PostgreSQL ledger. Build the foundational sync protocol that captures local mutations (writes/updates/deletes) while offline, queues them durably, and synchronizes them with the backend when the network connection is restored. Implement conflict resolution logic on the server to handle edge cases (e.g., inventory oversell). Ensure the frontend UI (375px width optimized) reflects optimistic updates instantly and displays a non-intrusive offline indicator. The sync process must be invisible to the user and require zero manual intervention. Do not prescribe the exact data schema or sync libraries; design a robust API contract and background worker process. Include E2E test coverage simulating a network drop, a local transaction, a network restore, and successful cloud ledger reconciliation.

  ## Priority
  P0

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []