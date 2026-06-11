issue_title: "[Research] Offline-First Mobile POS & Inventory Sync Engine for High-Latency Environments"
issue_description: |
  # Architectural Gap: Offline-First Mobile POS & Inventory Sync Engine

  ## Problem Statement
  For physical, in-person operations—such as Fatima’s food cart or Carlos’s field repair service—network connectivity is often unreliable, intermittent, or high-latency. Currently, OHC’s web and mobile interfaces assume a constantly available network connection to the backend API. If the network drops, operations like marking an order as complete, accepting an in-person payment (via Tap-to-Pay), or toggling a menu item to "sold out" can fail or hang.

  To serve the "Fatima" and "Carlos" personas effectively, OHC requires a robust Offline-First Sync Engine that allows critical operations to be queued locally in the mobile client and automatically synced to the multi-tenant PostgreSQL backend when the connection is restored, without throwing technical errors to the non-technical owner.

  ## Research Report (Track 1 & 2)
  - **Market Landscape**: Leading POS systems (Square, Shopify POS) heavily rely on robust offline modes. Square allows taking offline payments (within risk limits) and Shopify POS maintains a local cache of the product catalog.
  - **OHC Missing Capability**: While OHC supports mobile layouts (375px), it lacks a local-first SQLite/IndexedDB caching and mutation queue layer on the client side, coupled with a conflict-resolution sync mechanism on the Rust backend.
  - **Competitive Advantage**: By integrating AI agents into the sync process, the Operations Agent can intelligently notify the owner when back-online syncs cause conflicts (e.g., "Two customers ordered the last vegan cake while you were offline. I've drafted an apology message and a refund link for the second customer.").

  ## Design Doc (Track 3)
  ### Mobile UX Flow (375px Viewport)
  1. **Connectivity Indicator**: Subtle top-bar translucent pill showing "Working Offline" (no technical jargon like "Network Error").
  2. **Action Execution**: User taps "Mark Sold Out" on a product. The UI instantly updates to show it sold out, logging the mutation to a local queue.
  3. **Reconnection**: When online, the queue silently processes. If successful, no disruption. If conflicted, an Agent Card appears in the Workspace feed: "Sync Issue: Action needed on recent offline orders."

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Mobile as Flutter/PWA Client
      participant LocalStore as Client Local Store (Queue)
      participant Gateway as OHC Edge / Gateway
      participant SyncAPI as Rust Sync Engine (gRPC/REST)
      participant DB as Postgres (Row-Level Security)
      participant Agent as Operations Agent

      Mobile->>LocalStore: Mutate Item (Offline)
      LocalStore-->>Mobile: Optimistic UI Update
      Note over Mobile, Gateway: Network Restored
      Mobile->>SyncAPI: Push Mutation Queue (Idempotent)
      SyncAPI->>DB: Apply Changes & Check Versions
      alt Conflict Detected
          SyncAPI->>Agent: Trigger Conflict Resolution
          Agent->>Mobile: Generate Action Card
      else Success
          SyncAPI-->>Mobile: Sync Complete & State Revalidated
      end
  ```

  ### Core Components & Multi-Tenancy
  - **Client-Side Sync Queue**: A durable local store that records actions with `idempotency_key`, `timestamp`, `tenant_id`, and `action_payload`.
  - **Rust Backend Sync Engine**: An endpoint `POST /api/v1/sync/mutate` that processes ordered mutations. Uses PostgreSQL row-level locks and version columns (`xmin` or explicit `version`) to detect stale writes.
  - **Agent Hub Integration**: When standard conflict resolution fails, the system enqueues a job for the Operations Agent to review the business rules and draft a resolution for the owner.

  ## Implementation Prompt (Track 4)
  Implement the backend foundation for the Offline-First Sync Engine.

  **Outcome**: A new Rust service endpoint and database schema pattern capable of receiving, validating, and applying batched offline mutations for a tenant with conflict detection.

  **CUJ**: As an owner (Fatima), I toggle "Vegan Cake" to "Sold Out" while my phone has no signal. Five minutes later, I get signal, and the app seamlessly synchronizes this change to the backend without showing me any technical error screens.

  **Acceptance Criteria**:
  - Add a `version` or `updated_at` optimistic concurrency control column to a core entity (e.g., `inventory_items`).
  - Create a `POST /api/v1/sync/batch` endpoint in the Rust server that accepts an array of mutation operations (each with an idempotency key and expected previous version).
  - Implement logic to apply mutations using Postgres transactions, ensuring tenant isolation (`tenant_id`).
  - Return a structured response detailing which mutations succeeded and which failed due to conflicts.
  - Add 100% unit test coverage for the batch sync logic and conflict scenarios.
  - Add a Playwright E2E test that mocks a dropped network connection, performs a UI action, restores the network, and verifies successful sync.

  ## Top 5 Things That Do Not Make Sense (To Fix Later)
  1. The API backend currently mixes Go and Rust (README vs Prompt architecture instructions).
  2. The mobile app lacks a systematic offline-first persistence strategy, causing intermittent UI hangs on spotty networks.
  3. Tenant isolation relies on implicit application code checks in some areas rather than universal PostgreSQL RLS enforcement.
  4. The Legacy Next.js UI is still active and conflicting with the newer Flutter/Tauri directions.
  5. Error states in the app default to standard network errors instead of owner-friendly agentic actionable cards.

  ## Priority: P0
  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
