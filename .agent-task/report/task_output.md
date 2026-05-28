issue_title: "[Architecture] Invisible Offline-First CRDT State Synchronization Mesh"
issue_description: |
  # Research Report: Invisible Offline-First CRDT State Synchronization Mesh

  ## 1. Architectural Gap & Scaling Discovery

  ### Codebase & Docs Audit
  Our existing system heavily relies on an online-first architecture where actions on the client immediately attempt to sync with the central Postgres database or rely on queuing background tasks. However, many of our user personas operate in low-connectivity or intermittent-connectivity environments. While we have components like an offline transaction queue for Tap-to-Pay POS, there is a fundamental gap in general state synchronization across all application domains (inventory, ledger, bookings) when offline.

  ### Competitor Systems Audit
  Leading mobile-first POS systems and modern collaborative apps (e.g., Square, Linear) utilize robust offline-first architectures. They often employ Conflict-free Replicated Data Types (CRDTs) to ensure seamless local editing that deterministically resolves conflicts upon reconnection, without forcing the user to manually handle synchronization errors.

  ### Identified Gap
  OHC currently lacks a unified, invisible Offline-First CRDT State Synchronization Mesh. Maya (selling at a farmer's market with spotty 4G), Carlos (in a customer's basement with no signal), and Fatima (operating a food cart in a crowded area) all experience friction if the app blocks them from updating inventory, sending quotes, or managing bookings when offline. We need a system that treats the local SQLite store as the primary source of truth, replicating state seamlessly to the cloud when connectivity allows, powered by CRDTs to merge concurrent changes (e.g., a customer buys online while Maya sells the same item offline).

  ## 2. Selected Architecture Deep Dive

  ### Business Journey Mapping
  - **Persona Interaction:** Fatima is operating her food cart. Her phone loses connection. A customer walks up and orders. Fatima opens the app, taps the order items, and completes the sale. The app responds instantly.
  - **Invisible Sync:** When her phone regains connectivity, the local state (inventory decremented, transaction logged) syncs with the central cloud.
  - **Conflict Resolution:** If an online order came in for the last item simultaneously, the system uses CRDT logic (e.g., last-writer-wins, or appending to an event log) to resolve the state. The Operations Agent then automatically flags the over-sell and texts Fatima ("I noticed we oversold Item X due to a simultaneous online order. I've automatically refunded the online customer and offered them a 10% discount on their next order.").

  ### Data Model & Invariants
  The architecture revolves around distributed event logs and CRDT wrappers for core entities.

  ```mermaid
  erDiagram
      TENANT ||--o{ REPLICATION_LOG : owns
      REPLICATION_LOG ||--|{ CRDT_ENTITY : contains
      CRDT_ENTITY }|--|| LOCAL_SQLITE : stores
      CRDT_ENTITY }|--|| REMOTE_POSTGRES : syncs

      REPLICATION_LOG {
          string client_id
          bigint clock
          string operations_vector
      }

      CRDT_ENTITY {
          string entity_id
          string entity_type "e.g., INVENTORY, LEDGER"
          jsonb state_vector
      }

      LOCAL_SQLITE {
          string status "dirty_sync"
      }
  ```

  **Invariants & Multi-Tenant Rules:**
  - Multi-tenant isolation is enforced at the replication layer. The sync protocol authenticates via SPIFFE/OIDC and only synchronizes operations tagged with the authenticated `tenant_id`.
  - The local SQLite database encrypts data at rest using `OHC_SQLITE_KEY` ensuring data security on the device.

  ### AI Department Coordination
  - **Operations Agent:** Monitors the conflict resolution stream. If a hard conflict (e.g., overselling) occurs, it executes remediation playbooks.
  - **Customer Success (CS) Agent:** Proactively communicates with customers affected by conflict resolution (e.g., delayed bookings).

  ## 3. Technical Integrity & Mobile-First Review

  ### Mobile-First UX Flow
  - The user interface operates identically regardless of network state.
  - A subtle indicator in the top right (e.g., a cloud icon) changes state from "Connected" to "Working Offline".
  - Actions like "Save", "Checkout", or "Book" complete instantly (Optimistic UI) with a local success state.
  - Error states regarding connectivity ("Please connect to the internet to do this") are completely eliminated for core workflows.

  ### Performance & Offline Targets
  - Local database reads/writes must complete in < 50ms.
  - The sync payload must be compressed and batched to minimize data usage, suitable for degraded 3G/4G networks.
  - The replication protocol must gracefully handle frequent network interruptions (resumable sync).

  ### Zero Trust & Security
  - Sync endpoints on the Rust API server require JWT/OIDC authentication.
  - Client operations must be signed to prevent spoofing of operations.

  ## 4. Strategic Feature Issue Dispatch
  This research is formulated into the issue brief `[architecture]_invisible_offline_first_crdt_state_synchronization_mesh.md` for immediate prioritization.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []