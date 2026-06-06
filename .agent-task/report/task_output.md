issue_title: "Offline-First Resilient Sync for Low Connectivity Environments"
issue_description: |
  ## Problem Statement
  Fatima (50, non-technical) runs a halal food cart and operates in urban areas with spotty cellular coverage or slow data connections. She relies on a low-end Android phone. When connectivity drops, she currently cannot see new pre-orders or toggle menu items as "sold out". This leads to frustrated customers who pre-order sold-out items, or lost revenue from missed pre-orders. Existing SMB platforms typically require constant, stable network connectivity to function, which fundamentally fails the real-world mobile-first scenarios for street vendors, farmers' market sellers, and field service workers (like Carlos the handyman). We need a platform that works seamlessly regardless of network conditions.

  ## Research Report
  - **Competitive Landscape**:
    - **Shopify**: Shopify POS offers an offline mode, but it is primarily designed for physical hardware (card readers/iPads) and complex retail setups, rather than a single lightweight mobile app.
    - **Wix & Squarespace**: Both platform management apps degrade severely or become completely unusable when offline. They lack durable optimistic UI updates for core operations.
    - **Square**: Offers an offline mode for taking payments, but management tasks (like inventory updates) often require an active connection.
  - **Industry Best Practices**: Modern mobile-first architectures utilize a "Local-First" approach. Data is read from and written to a local embedded database (e.g., SQLite/Isar). A background synchronization engine handles the bidirectional syncing with the cloud backend whenever the network is available.
  - **AI Opportunity**: The AI Operations Agent can intelligently handle conflicts that arise from delayed syncs (e.g., automatically drafting an apology and refund if two customers ordered the last item while the device was offline).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant U as User (Fatima)
      participant F as Flutter App (Mobile)
      participant L as Local DB (SQLite)
      participant S as Sync Engine (Background)
      participant B as OHC Backend (Go/gRPC)
      participant DB as PostgreSQL (Tenant Data)

      U->>F: Toggles "Sold Out" on Falafel
      F->>L: Write state to Local DB (Optimistic)
      F-->>U: Immediate UI Update (Glassmorphism Card turns grey)
      F->>S: Enqueue Mutation Event
      loop Every Network Tick
          S->>B: Attempt Sync (with Exponential Backoff)
          alt Network Up
              B->>DB: Update State (Conflict Resolution)
              B-->>S: Ack & Sync Downstate
              S->>L: Clear Queue, Update Local State
              S-->>F: Remove "Offline" indicator
          else Network Down
              S-->>F: Display subtle amber "Sync Pending" dot
          end
      end
  ```

  ### UI & UX Flow (375px Mobile First)
  - **Top Navigation Bar**: A subtle, premium Glassmorphism indicator. When offline, a small amber dot appears with "Syncing paused" text. No intrusive banners or blocking loaders.
  - **Inventory/Menu Screen**: When Fatima toggles an item to "Sold Out", the native toggle switch flips immediately. The item card gracefully transitions to a lower opacity state with a 20px blur backdrop-filter.
  - **Action Queueing**: The user is completely shielded from network errors. There are no "Network Error, Try Again" popups. All actions are silently queued.

  ### Key Design Decisions
  1. **Durable Local Queue**: Use SQLite (or Isar) on the Flutter client to store a durable queue of mutation intents.
  2. **Optimistic UI**: The UI must reflect local mutations instantly. The network layer is entirely decoupled from the UI rendering layer.
  3. **Conflict Resolution Strategy**: Server is the ultimate source of truth, but we use Timestamp-based Last-Write-Wins for simple toggles, and CRDT-like operations for counters (like remaining stock).
  4. **Agent Handoff**: If a delayed sync causes a real-world conflict (e.g., overselling), the `Operations Agent` is triggered to handle the customer communication automatically.

  ## Implementation Prompt
  **Task for Implementer:**
  Implement the Offline-First Resilient Sync Engine for the Flutter mobile application and the corresponding Go backend conflict resolution handlers.
  - Create a `SyncManager` service in Flutter that intercepts all critical API calls (e.g., updating inventory, confirming orders).
  - If offline, `SyncManager` must write the action to a local SQLite queue and return success to the UI (Optimistic UI).
  - Implement a background worker in Flutter that flushes this queue to the Go backend (`/api/v1/sync`) using exponential backoff when connectivity is restored.
  - On the Go backend, implement the `/sync` endpoint to process batched operations, applying timestamp-based conflict resolution and updating PostgreSQL. Ensure tenant isolation.
  - Add Playwright E2E tests simulating an offline network state, performing an action, restoring the network, and verifying the backend state updates correctly.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
