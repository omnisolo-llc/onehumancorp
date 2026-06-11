issue_title: "Implement Real-time Agent Action Feed via WebSocket"
issue_description: |
  # Real-time Agent Action Feed (WebSocket Architecture)

  ## Title
  Implement Real-time Agent Action Feed via WebSocket

  ## Problem Statement
  Currently, OneHumanCorp (OHC) operates on a "pull" model where business owners like Maya (the baker) or Carlos (the field service owner) have to open the app and manually refresh to see if an AI agent has drafted a response to a customer DM or generated a new quote. This defeats the purpose of an "assistant." An assistant should tap you on the shoulder when there is something actionable. The lack of real-time push capability prevents the "Agent Feed" (our core UX paradigm) from functioning correctly on the 375px mobile viewport, causing owners to miss critical, time-sensitive approvals (like intercepting a high-value order inquiry).

  ## Research Report
  - **Competitive Baseline**: Tools like Intercom Fin and HubSpot Breeze rely on real-time messaging layers (WebSockets/SSE) to push agent interactions instantly.
  - **Market Validation**: Our internal research (`agent_feed_deep_dive.md`) highlights that the "Agent Feed" is the central nervous system. It requires proactive pushing of "Action Cards" directly to the user's mobile device.
  - **Technical Gap**: The current Rust backend (`src/server`) has robust polling/REST APIs but lacks a dedicated, multi-tenant WebSocket or Server-Sent Events (SSE) infrastructure to stream agent events (e.g., `AgentDraftReady`, `CustomerMessageReceived`) directly to the Flutter/Tauri clients in real-time.
  - **Why not polling?** Mobile networks (Carlos in the field) suffer under heavy polling (battery drain, high data usage). WebSockets offer a persistent, low-overhead channel crucial for the "low-data mode" requirement.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Mobile Client (Tauri/Flutter)
      participant WS as Real-time Gateway (Rust/Axum)
      participant PubSub as Redis (Valkey)
      participant Agent as AI Agent Worker

      Owner->>WS: Connect (WSS) w/ JWT (Tenant ID)
      WS->>PubSub: Subscribe to `ohc:feed:{tenant_id}`
      Agent->>Agent: Generate Draft for Customer DM
      Agent->>PubSub: Publish `ActionCard` Event to `ohc:feed:{tenant_id}`
      PubSub->>WS: Deliver Event
      WS->>Owner: Push JSON payload over WebSocket
      Owner->>Owner: Render Action Card in UI
  ```

  ### Mobile UX Flow (375px)
  1. User opens the app. The "Today's Priorities" feed is visible.
  2. A WebSocket connection is established silently in the background.
  3. When an agent finishes a task (e.g., drafts a reply to an Instagram DM), the backend pushes an event.
  4. The UI seamlessly animates a new "Action Card" to the top of the feed with a subtle "ping" animation (using translucent materials).
  5. The card has clear, touch-friendly (44x44px min) buttons: "Approve", "Edit", "Discard".

  ### AI Agent Integration
  - The AI Agent Workers (background jobs) do not talk to clients directly. Upon completing a task that requires human approval, they publish a standardized JSON payload to the Valkey (Redis) Pub/Sub channel scoped exactly to the `tenant_id`.

  ### Key Design Decisions
  - **Protocol**: WebSockets (using `tokio-tungstenite` or Axum's built-in `ws` extraction) over SSE, to allow future bi-directional "quick replies" directly over the socket.
  - **Multi-tenancy/Security**: The WebSocket handshake MUST validate the tenant JWT. The resulting connection only subscribes to the Redis topic specific to that `tenant_id` to prevent data leakage.
  - **State Management**: The WebSocket is transient. Missed messages must be recoverable via a standard REST fetch on reconnect.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to build the Real-time Agent Action Feed infrastructure.
  1. Add a WebSocket endpoint (e.g., `/api/v1/feed/ws`) in the Rust Axum server.
  2. Implement JWT-based authentication during the WebSocket handshake to extract the `tenant_id`.
  3. Connect the authenticated WebSocket session to a Valkey (Redis) Pub/Sub subscription specific to that `tenant_id`.
  4. Create a mechanism for AI Agent workers to publish `ActionCard` events to the corresponding Redis channel.
  5. **Acceptance Criteria**: A client connecting to the WebSocket should receive real-time JSON pushes when an agent publishes an event for their tenant. Disconnections should be handled cleanly. Multi-tenant isolation is absolutely critical.
  6. Remember to adhere to the zero-trust SPIFFE/SPIRE context where applicable and ensure the feature handles offline/reconnects gracefully on mobile clients. Write thorough unit and E2E tests validating the real-time push.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
