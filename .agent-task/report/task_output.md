issue_title: "[Architecture] Real-Time Multi-Tenant Edge Notifications & Sync"
issue_description: |
  # Real-Time Multi-Tenant Edge Notifications & Sync

  ## Problem Statement
  Owners and operators currently lack real-time visibility into mission-critical events (e.g., new orders, support requests, AI agent actions). They must pull to refresh or manually check the UI, which defeats the purpose of an "assistant-first" and "owner-centered" platform. We need a way to push real-time updates to mobile and web clients across all 375px+ interfaces reliably, gracefully degrading to polling if necessary, without sacrificing multi-tenant security.

  ## Research Report
  Leading platforms (Shopify, Stripe, Slack) use a combination of WebSockets, Server-Sent Events (SSE), and intelligent polling to push updates to clients. In a multi-tenant environment, the connection must be strictly bound to the `tenant_id` and securely authenticated via SPIFFE/SPIRE principles. We need to implement a scalable notification mechanism leveraging our existing stack (Go + Bazel, PostgreSQL, Redis). A Redis Pub/Sub backplane is ideal for scaling WebSocket/SSE connections across multiple Go API instances.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Client as Web/Mobile App (Flutter)
      participant API as Go API Gateway
      participant Auth as Auth/SPIRE
      participant Redis as Redis Pub/Sub
      participant DB as PostgreSQL
      participant Worker as AI Job Worker

      Client->>API: Connect (WebSocket/SSE) + Token
      API->>Auth: Validate Token & get tenant_id
      Auth-->>API: tenant_id
      API->>Redis: Subscribe to tenant_events:{tenant_id}
      Worker->>DB: Perform Action (e.g., Process Order)
      Worker->>Redis: Publish to tenant_events:{tenant_id} (Event Data)
      Redis-->>API: Event Received
      API-->>Client: Push Event
  ```

  ### Mobile UX Flow (375px first)
  - The Assistant shell maintains a persistent connection.
  - Incoming events trigger a non-intrusive toast notification or update badge counts.
  - Critical alerts (e.g., "New pre-order for pickup in 10 mins") can bypass standard Do-Not-Disturb logic if the owner has configured it.
  - If the connection drops, the client seamlessly falls back to periodic polling and re-syncs state upon reconnection.

  ### AI Agent Integration Points
  - Agents can emit structured events (e.g., `agent.task.completed`, `agent.draft.ready`) that are pushed immediately to the owner.
  - The Triage capability can push real-time prioritization updates to the owner feed.

  ### Key Design Decisions
  - **Transport**: Prioritize WebSockets for low latency bidirectional communication (useful if we expand to interactive agent chat). SSE as a fallback.
  - **Backplane**: Redis Pub/Sub provides a stateless, horizontally scalable way to route events to the correct API node holding the client connection.
  - **Security**: WebSocket connections must undergo the same strict tenant isolation and token validation as REST/gRPC calls. The topic name MUST include the `tenant_id` (`tenant_events:{tenant_id}`).
  - **Graceful Degradation**: The client must gracefully handle reconnections and synchronize missed events via a standard REST API call on reconnect.

  ## Implementation Prompt
  Implement a real-time event notification system for OHC clients. The system must use a Redis Pub/Sub backplane to route events to connected clients via WebSockets or Server-Sent Events. Ensure strict tenant isolation: clients must only receive events for their authenticated `tenant_id`. Develop the server-side infrastructure in Go and provide a resilient client-side integration pattern that handles reconnections seamlessly. Create an E2E test verifying that an action triggered by a worker (or another API call) results in a real-time push to the connected test client.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
