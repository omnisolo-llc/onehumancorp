issue_title: "Implement Real-Time AI Agent Message Streaming & Handoff Orchestration"
issue_description: |
  ## Title: Unified Real-Time AI Agent Message Streaming & Handoff Orchestration

  ## Problem Statement
  Currently, owners interacting with the AI assistant through the Flutter mobile and PWA frontend experience high latency between their request and the final resolution. Agents operate entirely synchronously or fail silently in the background, leaving the owner (like Maya the baker or Fatima the food cart operator) staring at a loading spinner without visibility into what the AI is actually doing. To provide a "Tencent Workbuddy-like" trusted assistant experience, the owner needs real-time, token-by-token feedback as agents process their requests, as well as distinct status signals when work is handed off between different AI departments (e.g., Sales drafting a proposal while Operations creates project tasks).

  ## Research Report
  ### Context & Competitor Analysis
  - **Shopify Sidekick / Wix AI:** Provide instant visual feedback via streaming chat interfaces. They use WebSocket or Server-Sent Events (SSE) to push updates from background job queues directly to the client.
  - **Current OHC Codebase:** The Axum backend uses traditional request-response for many AI interactions, leading to timeouts on complex agentic workflows. There is no unified pub/sub layer actively pushing events to the `tenant_id` clients.
  - **Proposed Shift:** Implementing an SSE (Server-Sent Events) endpoint in the Axum backend that subscribes to a Redis Pub/Sub channel. The AI job queue (using PostgreSQL `SKIP LOCKED`) will publish progress events to Redis, which the Axum SSE route streams to the Flutter UI.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Mobile UI (Flutter)
      participant Axum as API Layer (Axum SSE)
      participant Redis as Redis Pub/Sub
      participant Queue as AI Job Worker (PostgreSQL)
      participant LLM as Gemini Pro

      Owner->>Axum: Connect to /api/v1/agents/stream
      Axum->>Redis: Subscribe (channel: tenant_{id})
      Owner->>Axum: POST /api/v1/agents/ask (Task: "Quote custom cake")
      Axum->>Queue: Enqueue Task
      Axum-->>Owner: 202 Accepted
      Queue->>LLM: Stream completion
      LLM-->>Queue: Token chunks
      Queue->>Redis: Publish (channel: tenant_{id}, event: token)
      Redis-->>Axum: Forward event
      Axum-->>Owner: SSE: data: {"token": "..."}
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  1. **Initial State:** The assistant chat interface at 375px width. Native mobile keyboard visible.
  2. **Active State:** User submits a request. The message bubble appears immediately. Below it, a translucent glass container (OHC Premium Token) appears with an animated gradient border indicating "Agent is thinking...".
  3. **Streaming State:** Tokens appear in real-time within the glass container. If a handoff occurs, a small inline status pill appears (e.g., "Operations Agent taking over...").
  4. **Completion:** The glass container transitions to a solid message bubble, and a suggested action button (e.g., "Approve Quote", 44x44px touch target) slides in.

  ### AI Agent Integration Points
  - **Department Orchestrator:** Must be updated to emit lifecycle events (`agent_started`, `department_handoff`, `token_yield`, `agent_completed`).
  - **Redis Pub/Sub:** Events published with the strict naming convention `ohc:stream:{tenant_id}` to ensure Zero Trust multi-tenant isolation.

  ### Key Design Decisions
  - **Why SSE over WebSockets:** SSE is strictly unidirectional (server-to-client) which perfectly matches the required token streaming behavior, has built-in reconnection, and avoids the complexity of full-duplex WebSocket state management over flaky mobile networks.
  - **Zero Trust Multi-Tenancy:** The SSE Axum route MUST validate the JWT and only subscribe to the specific `tenant_id` Redis channel. Cross-tenant leakage is prevented at the connection layer.

  ## Implementation Prompt
  **User-Facing Outcome:** When Maya asks the assistant to draft a response to a custom cake inquiry, she sees the text streaming in real-time rather than waiting 15 seconds for a single block of text.

  **CUJ & Acceptance Criteria:**
  1. Create an Axum SSE route (`/api/v1/agents/stream`) that authenticates the user, extracts `tenant_id`, and subscribes to `ohc:stream:{tenant_id}` in Redis.
  2. Update the AI Job Queue worker to publish streaming tokens and agent state changes to the Redis Pub/Sub channel.
  3. Ensure events strictly conform to a `StreamEvent` JSON structure containing event type, payload, and timestamp.
  4. Implement a Playwright E2E test verifying that a simulated backend event is correctly received by the frontend connection.
  *Note: Do not prescribe the specific Rust channel internal implementation, but ensure Redis is used for multi-node scaling.*

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
