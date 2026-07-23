issue_title: "Implement High-Performance Agent Feed Batching & Offline Sync Sync for Mobile Clients"
issue_description: |
  # OHC Mobile Agent Feed Synchronization Architecture Design

  ## Problem Statement
  Our primary owner personas—like Fatima (Food Cart Operator on slow mobile data) and Carlos (Field Service Owner moving through spotty coverage areas)—rely entirely on the OHC mobile application to manage their operations via the Unified Agent Feed.
  Currently, if they lose connectivity, real-time agent proposals (e.g. "Customer requested an estimate", "Inventory low, reorder?") either fail to deliver or cause jarring UI shifts upon reconnection. The existing WebSocket implementation in `agent_feed.rs` provides basic batching, but lacks robust offline capabilities, message deduplication, and a structured sync protocol to guarantee zero data loss during network transitions. The mobile user experience degrades rapidly on unreliable networks.

  ## Research Report
  - **Market Mapping**: Modern field-service and mobile-first management tools (like Field Nation, Jobber, and Shopify POS) employ aggressive local-first data caching and strict synchronization protocols to survive offline scenarios.
  - **OHC Gap**: While OHC uses Redis Pub/Sub to deliver real-time agent events to WebSocket clients, mobile clients lacking connection miss these messages. Upon reconnection, there is no reliable backfill mechanism beyond re-fetching the entire feed, and rapid reconnections can cause duplicate processing of Agent Feed Action Cards.
  - **Business Impact**: Missing an agent notification (like an urgent custom cake request for Maya) directly impacts revenue.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Mobile as OHC Mobile App (Offline/Online)
      participant API as Agent Feed Sync Endpoint
      participant Redis as Valkey (Pub/Sub & Event Store)
      participant DB as Central Ledger (Postgres)
      participant Agent as OHC AI Agents

      Agent->>DB: Persists new Feed Item
      Agent->>Redis: Publishes "FeedUpdate" Event (with sequence ID)
      alt Client Offline
          Mobile-->>API: Connection Lost
          Redis->>Redis: Appends event to Tenant Stream (Event Store)
      else Client Reconnects
          Mobile->>API: WS Connect + Last Sync SeqID
          API->>Redis: Fetch missed events > SeqID
          API->>Mobile: Sends Batched Backfill payload
          Mobile->>Mobile: Deduplicates & Caches locally
      end
  ```

  ### Mobile UX Flow
  - **Offline State**: Feed cards are stored locally in SQLite/Hive on the device. User actions (e.g., hitting "Approve" on a draft) are queued locally and visually marked as "Pending Sync" (using translucent UI tokens).
  - **Reconnection State**: The app establishes the WebSocket connection, passing its `last_sequence_id`. The server immediately flushes the backlogged events in a single compressed batch.
  - **Zero UI Jitter**: Deduplication ensures that cards aren't re-rendered. The UI smoothly transitions "Pending Sync" items to "Approved".

  ### AI Agent Integration Points
  - Agents (e.g. The Ambassador, Operations Agent) publish their drafts and alerts to both PostgreSQL (for permanence) and a Valkey Redis Stream (for ordered synchronization).
  - The Sync API acts as the bridge between the Redis Stream and the Mobile WebSocket, managing the offset (sequence ID) for each mobile client session.

  ### Key Design Decisions
  - **Use Redis Streams instead of pure Pub/Sub**: Transitioning from `PUBLISH` to `XADD` allows the system to maintain an ordered log of events per tenant, enabling clients to request exactly what they missed during a network drop.
  - **Client-Side Deduplication**: The mobile client handles idempotency using the unique Feed Item ID to prevent duplicate action cards.

  ## Implementation Prompt
  **User-Facing Outcome**: When Fatima drives through a tunnel and loses service, the app continues to display her Agent Feed. Any actions she takes are queued. When she regains service, the app seamlessly catches up on missed agent alerts and syncs her actions without any loading spinners or duplicate cards.

  **Acceptance Criteria**:
  1. Replace the ephemeral Redis Pub/Sub in the Agent Feed WebSocket with a reliable Redis Stream (or equivalent ordered log) per tenant.
  2. Update the WebSocket connection handler to accept a `last_sequence_id` from the client.
  3. On connection, the server must query the stream for any events newer than `last_sequence_id` and send them to the client as an initial backfill batch.
  4. Ensure the system handles rapid reconnects gracefully without flooding the client.
  5. Provide a REST fallback endpoint for clients to poll their missed events if WS fails.
  6. E2E test verifying that a simulated offline period followed by a reconnection successfully delivers the delayed events.

  ## Priority & Scope
  - **Priority**: P1
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
