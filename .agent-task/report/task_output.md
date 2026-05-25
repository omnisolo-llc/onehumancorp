issue_title: "Autonomous AI-Driven Offline-First Sync Architecture"
issue_description: |
  # [architecture] Autonomous AI-Driven Offline-First Sync Architecture

  ## Problem Statement
  Small business owners like Carlos (handyman) and Fatima (food cart) often work in areas with spotty or zero internet connectivity. Currently, if their phone drops connection, they cannot accept payments, update their inventory, or receive new orders seamlessly. This introduces critical friction in their daily operations and risks lost revenue. We lack a truly offline-first, AI-driven synchronization mechanism that allows continuous operation without an active connection and intelligently reconciles state when the connection is restored, handling conflicts without user intervention.

  ## Research Report
  - **Shopify POS:** Offers limited offline mode (cash/custom payments), but relies heavily on online sync for inventory and card payments.
  - **Square POS:** Supports offline payments but stores sensitive card data locally until reconnected. Inventory often gets out of sync during extended offline periods.
  - **Our Gap:** OHC requires an invisible, multi-tenant offline-first architecture that uses Local-First software principles (CRDTs - Conflict-free Replicated Data Types) combined with a local lightweight AI Agent running on the edge device to handle complex offline conflicts (e.g., overselling inventory).

  ## Design Doc
  The solution relies on a Local-First CRDT Data Layer syncing with our Backend NATS Event Mesh when online.

  ### Architecture Diagram
  ```mermaid
  graph TD
      MobileApp[Mobile App - React Native / Web] --> LocalAgent[Edge AI Agent]
      MobileApp --> LocalStore[Local CRDT Database]
      LocalAgent --> LocalStore
      LocalStore -- "Background Sync (when online)" --> CloudMesh[NATS Hybrid Event Mesh]
      CloudMesh --> CloudStore[Cloud Postgres / Multi-Tenant]
      CloudAgent[Cloud AI Conflict Resolver] --> CloudStore
      CloudMesh -. "State Updates" .-> LocalStore
  ```

  ### Mobile UX Flow
  - **375px First:** The user interface remains identical whether online or offline. A subtle "Translucent Glass" indicator pill at the top of the dashboard shows connection status (e.g., "Working Offline - All saved").
  - **Action:** Fatima rings up an order while offline. The app responds instantly. The Edge AI agent caches the event.
  - **Reconnection:** Once back online, the sync pill spins invisibly. Any conflicts (e.g., two offline devices sold the last item) are resolved by the Cloud AI Conflict Resolver based on business rules (e.g., prioritize the first local timestamp).
  - **Grandmother Test:** Fatima doesn't need to know what "syncing" or "offline" means. She just taps "Charge" and it always works.

  ### AI Agent Integration Points
  - **Edge AI Agent:** Runs lightweight WASM-based logic to validate offline actions (e.g., ensuring a booking slot is likely free based on cached patterns).
  - **Cloud AI Conflict Resolver (Operations Dept):** Triggers upon sync if CRDTs cannot resolve a conflict automatically. It reviews the business's policy (e.g., "offer a discount if oversold") and executes it invisibly.

  ### Key Design Decisions
  - **CRDTs for State:** Using CRDTs guarantees eventual consistency without locking.
  - **Strict Multi-Tenant Isolation:** All CRDT vectors and Sync tokens are cryptographically bound to the SPIFFE/SPIRE identity of the specific `tenant_id` and device.
  - **Offline Payments:** Safely queue encrypted payment tokens to be processed via Stripe/Payment provider when connectivity resumes, following PCI compliance guidelines.

  ## Implementation Prompt
  **Role:** Implementer Agent
  **Task:** Build the core synchronization engine for the Offline-First CRDT Data Layer.
  **Requirements:**
  1. Create the `SyncEngine` module that interfaces between the Local Device State and the NATS Hybrid Event Mesh.
  2. Implement the conflict resolution queue that the Cloud AI Conflict Resolver will monitor.
  3. Ensure all sync operations strictly enforce Zero-Trust multi-tenant isolation (validate `tenant_id` via SPIFFE/SPIRE on every incoming sync payload).
  4. Build the mobile UI translucent connection indicator component following the macOS-style design tokens.
  **Acceptance Criteria:**
  - Devices can create, update, and delete entities while completely offline.
  - State synchronizes automatically to the cloud within 2 seconds of connection restoration.
  - The UI accurately reflects offline/syncing states without blocking user interaction.
  - Cross-tenant data leakage is prevented via strict identity validation.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
