issue_title: "[research] Architect Real-Time Multi-Tenant Notification Bus for AI Work Assistants"
issue_description: |
  # Research Report: Real-Time Multi-Tenant Notification Bus

  ## Track 1: Architectural Gap & Scaling Discovery
  Our OHC platform provides an incredible AI-assisted management experience. However, a major architectural gap holding back our core personas (Maya, Carlos, Priya, Leo, Fatima) is the lack of a real-time, robust, and scalable multi-tenant notification bus.

  Currently, actions performed by AI agents in the background (e.g. drafting a reply, securing a booking, calculating tax, syncing inventory) rely on passive polling or unscalable database triggers. Competitors like Shopify and Stripe excel because their real-time events push updates instantly to their web/mobile dashboards and external webhook subscribers.

  For OHC to feel like a "living" work assistant, we must implement a centralized pub/sub event bus that pushes state changes (via WebSockets or Server-Sent Events) instantly to the 375px mobile client, completely eliminating the need for manual refreshes.

  ## Track 2: Selected Architecture Deep Dive
  **Business Journey:** Maya the Baker is running her shop. When an AI agent drafts a reply to an Instagram DM, Maya's phone should instantly pop up a floating notification or update her feed without any action on her part.

  **System Design:**
  1. **Event Source:** PostgreSQL `LISTEN/NOTIFY` (for DB-level state changes) and Redis Pub/Sub (for ephemeral agent actions).
  2. **Notification Service (Go):** A lightweight gRPC/WebSocket gateway that subscribes to Redis/PostgreSQL and multiplexes events to connected clients.
  3. **Multi-Tenancy:** Each WebSocket connection must authenticate via JWT (verifying tenant ID) and only subscribe to channels matching its `tenant_id`. Redis channels will use the pattern: `ohc:events:{tenant_id}:{resource}`.

  ```mermaid
  sequenceDiagram
      participant App as Mobile App (Flutter/PWA)
      participant Gateway as WebSocket Gateway (Go)
      participant AI as Agent Worker
      participant Redis

      App->>Gateway: Connect (JWT Token with tenant_id)
      Gateway->>Redis: Subscribe(ohc:events:{tenant_id}:*)
      AI->>Redis: Publish(ohc:events:{tenant_id}:drafts, payload)
      Redis->>Gateway: Event Data
      Gateway->>App: Push Event
      App->>App: Optimistic UI Update / Notification
  ```

  ## Track 3: Technical Integrity & Mobile-First Review
  - **Mobile-First UX:** The real-time events will power subtle, non-intrusive "toast" notifications (translucent glass style) and auto-updating list feeds on the 375px viewport.
  - **Performance/Offline:** If the WebSocket disconnects, the mobile client must queue events and perform a standard HTTP REST sync upon reconnection, ensuring eventual consistency.
  - **Security:** Strict Zero-Trust isolation. The WebSocket Gateway must independently validate the JWT signature and reject any subscription requests for foreign `tenant_id`s.

  ## Track 4: Strategic Feature Issue Dispatch
  **Implementation Prompt for Engineering Swarm:**
  - **Goal:** Implement the backend WebSocket Gateway in Go and the corresponding frontend subscription hook.
  - **Backend:** Create a Go service that listens to Redis Pub/Sub on `ohc:events:{tenant_id}:*` and pushes to connected authenticated WebSockets.
  - **Frontend:** Implement a React/Flutter hook to establish the WebSocket connection, handle reconnects, and dispatch events to the local UI state.
  - **E2E Test:** Write a Playwright test where a backend mock triggers a Redis publish event, and the UI verifies the event is rendered on the screen within 2 seconds.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
