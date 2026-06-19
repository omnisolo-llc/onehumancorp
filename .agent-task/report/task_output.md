issue_title: "Implement Multi-Tenant Real-Time AI Agent Message Streaming"
issue_description: |
  # Research Report: Real-Time AI Agent Communication & State Streaming

  ## 1. Problem Statement
  OneHumanCorp's core value proposition is an "Owner Work Assistant" where users (e.g., Maya the baker, Nora the agency principal) interact with an AI that coordinates actions invisibly in the background. Currently, when an owner initiates a long-running agent task (like analyzing a batch of incoming emails, drafting multiple proposals, or generating a complex schedule), the frontend often has to wait for a monolithic API response. This blocks the UI, breaks the illusion of a responsive "work assistant," and leaves the user staring at a loading spinner without understanding what the agent is actively doing. If the network drops or the connection times out, the work state is lost or unclear.

  We need a real-time, robust, multi-tenant streaming capability so the UI can display the agent's thought process, partial draft responses, and background tool execution (e.g., "Drafting proposal...", "Checking availability...") in real time. This aligns with the "Full-Spectrum Observability" core value.

  ## 2. Research Report
  - **Market Context**: Platforms like ChatGPT, Claude, and specialized agentic platforms (like Intercom's Fin) use Server-Sent Events (SSE) or WebSockets to stream token-by-token generation and agent state changes. This is industry standard for AI interactions to maintain user trust and engagement.
  - **The OHC Opportunity**: By implementing real-time streaming, the OHC assistant moves from a slow, transactional "bot" to a transparent, collaborative "partner." Maya can see the agent read an inquiry, look up inventory, and begin drafting a response sequentially.
  - **Competitive Landscape**:
    - *Shopify Sidekick*: Streams responses but operates mainly as a Q&A tool rather than an autonomous action-taker.
    - *HubSpot AI*: Often relies on asynchronous batch processing with email notifications for long tasks.
    - *Notion AI*: Streams text generation directly into the editor.

  ## 3. Design Doc
  ### Architecture Diagram (Concept)
  ```mermaid
  sequenceDiagram
      participant Owner_UI as Frontend (Flutter/PWA)
      participant API as Go Server API
      participant AI_Job_Queue as AI Job Queue (Postgres/Redis)
      participant Agent_Worker as Go Agent Worker
      participant LLM as LLM Provider

      Owner_UI->>API: POST /api/agents/chat (Task Request)
      API->>AI_Job_Queue: Enqueue Task
      API-->>Owner_UI: Return Job ID / Streaming Connection Open
      Agent_Worker->>AI_Job_Queue: Dequeue Task
      Agent_Worker->>LLM: Invoke Provider (stream=true)

      loop Streaming Generation
          LLM-->>Agent_Worker: Token/State Update
          Agent_Worker->>Redis: Publish Update (Channel: tenant_id:job_id)
          API-->>Owner_UI: Stream Event (SSE/WebSocket)
      end

      Agent_Worker->>Postgres: Save Final State
      Agent_Worker->>Redis: Publish Complete
      API-->>Owner_UI: Stream Complete Event
  ```

  ### Mobile UX Flow (375px)
  1. **User Action**: The owner inputs a command (e.g., "Draft a reply to the 3 new catering requests").
  2. **Initial State**: The chat interface immediately shows an "Agent is thinking..." bubble.
  3. **Streaming State**: The bubble updates dynamically with granular status tokens: "Reading inquiries..." -> "Checking calendar..." -> "Drafting responses...".
  4. **Token Generation**: The actual text of the drafted response streams in token-by-token within a translucent glass card.
  5. **Completion**: The card solidifies, presenting "Approve", "Edit", or "Discard" actions.

  ### Key Design Decisions
  - **Transport**: Use Server-Sent Events (SSE) or WebSockets in the Go API layer. SSE is simple for unidirectional text streaming over HTTP/2, but WebSockets might be chosen if bidirectional streaming is required in the future.
  - **State Management**: Leverage Redis Pub/Sub for inter-process communication between Go background workers processing the LLM streams and the Go API nodes holding the open SSE/WebSocket connections to the Flutter clients.
  - **Multi-Tenant Isolation**: Crucially, Redis channels and SSE subscription endpoints must strictly validate `tenant_id` claims using the SPIFFE/SPIRE identity infrastructure to ensure an owner never sees another tenant's agent streams.

  ## 4. Implementation Prompt
  **Feature Name**: Real-Time Agent Streaming Infrastructure
  **Target Persona**: All personas (e.g., Nora the agency principal waiting for a proposal draft).
  **Outcome**: When an owner interacts with the OHC Assistant, they receive real-time, token-by-token streaming of the agent's response and granular status updates of the agent's background actions.

  **Next Actions (for Implementer)**:
  1. Design and implement a real-time streaming endpoint in the Go backend API.
  2. Implement a publish/subscribe mechanism (using Redis) to route streaming tokens and agent state updates from Go background workers to the correct API node and connection.
  3. Ensure strict multi-tenant isolation; verify that a connection can only subscribe to streams belonging to its authenticated `tenant_id` utilizing standard identity.
  4. Create a generic Flutter/Dart frontend hook or service to consume this stream and update the UI reactively.
  5. Write Playwright E2E tests to verify the stream initiates, delivers tokens, and completes correctly within the UI. Ensure Bazel targets are fully tested.

  **Acceptance Criteria**:
  - The UI updates in real-time (token-by-token or state-by-state) without requiring manual polling.
  - Redis Pub/Sub channels (or equivalent) enforce `tenant_id` isolation.
  - Unit tests for Go backend and Flutter frontend cover the new logic.
  - E2E Playwright tests verify the streaming behavior.

  ## 5. Priority & Scope
  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
