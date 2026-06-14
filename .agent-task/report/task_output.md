issue_title: "Implement KAIROS Hub Teammate Mesh Architecture"
issue_description: |
  # Architecture Document: KAIROS Hub Teammate Mesh

  ## Problem Statement
  While OneHumanCorp provides multi-agent orchestration via KAIROS, a core architectural gap is the lack of a standardized Hub topology to cleanly isolate inter-departmental agent communications via a "Teammate Mesh". Currently, we have `src/server/orchestration/mesh.rs` and `src/server/orchestration/hub.rs`, but we lack a comprehensive mesh layer with real-time broadcast APIs and deep integration for inter-agent delegation and state transitions. As SMB users (like Maya the baker) scale their operations, they need AI agents across distinct departments (e.g., Marketing and Operations) to securely and reliably sync context, without database polling causing delays or performance issues.

  ## Research Report
  Our competitive analysis indicates that basic setups in Shopify or Wix do not provide native multi-agent coordination (often relying on disjointed third-party app webhooks). OHC's value proposition relies on treating AI as infrastructure. We have an existing KAIROS foundation and shared tasks, but the system lacks robust hub infrastructure to connect them at scale natively for cross-department handoff.

  Competitor Systems Audit:
  - **Shopify:** Operates via external webhooks and isolated third-party apps for separate workflows, causing high latency for any multi-step automation. Lacks a native pub/sub mesh for agents.
  - **Wix:** Basic automations exist, but they are rigid rule-based triggers rather than autonomous agents sharing real-time context.
  - **OHC Opportunity:** By building a dedicated "Teammate Mesh", OHC agents can communicate with sub-millisecond latency. This allows true concurrent agent execution (e.g. Operations Agent updates inventory, instantly broadcasting to the Marketing Agent to update an ad campaign) bypassing slow, traditional database polling.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Marketing Agent] -->|Teammate Mesh Broadcast| B(Redis PubSub / Channel)
      B --> C[Operations Agent]
      A -->|State Transition| D(State Machine)
      C -->|Verify Delegation| D
      B -.->|Tenant Context Isolation| E[(PostgreSQL)]
  ```

  ### Mobile UX Flow (375px First)
  *   **Mobile Dashboard (375px):** A "Teammate Sync" indicator in the Glassmorphism UI showing when agents are communicating (e.g., "The Promoter is briefing The Manager").
  *   **Agent Interaction Log:** A read-only, human-readable log (card-based UI) for the business owner to monitor inter-agent delegation and context sharing, providing transparency without technical jargon.

  ### AI Agent Integration Points
  *   **Inter-Departmental Handoff:** Agents use the mesh to securely pass structured context (e.g., Lead captured by Sales -> Handed off to Operations for onboarding).
  *   **State Machine Synchronization:** Events emitted over the mesh transition the centralized KAIROS state machine, ensuring verifiable and durable task completion.

  ### Key Design Decisions
  *   **Hybrid Transport Layer:** Use Redis Pub/Sub for scalable cloud deployments, falling back to a local, in-memory event bus (Channels/DashMap) for offline/standalone execution, ensuring the codebase remains unified across deployment targets.
  *   **Zero-Trust Isolation:** Implement strict row-level security and tenant_id boundaries within the mesh broadcast logic to prevent any data leakage between different businesses on the platform.

  ## Implementation Prompt

  **Feature Name:** KAIROS Hub Teammate Mesh APIs

  **Target Persona:** Maya the Baker (As she scales, her marketing and operations bots need to talk instantly).

  **User-Facing Outcome:** Business owners see near-instantaneous coordination between their AI agents (e.g., a customer DM leads to an instant inventory check and order creation) reflected live in their mobile dashboard without page refreshes.

  **Critical User Journey (CUJ):**
  1. An orchestrator agent generates a multi-step plan.
  2. The orchestrator delegates a sub-task to the Marketing Agent.
  3. The Marketing Agent completes the task and broadcasts the success via the `mesh:tasks` channel.
  4. The Hub receives the broadcast and updates the centralized state machine.
  5. The Operations Agent, subscribed to the mesh, instantly receives the state change and begins its dependent task.
  6. The mobile app UI updates in real-time to reflect the new active agent.

  **Acceptance Criteria:**
  - Implement the Realtime Teammate Mesh APIs in `src/server/orchestration/`.
  - Expose a `POST /api/mesh/v2/broadcast` endpoint (or enhance existing ones) for publishing events.
  - Implement a hybrid transport layer (Redis Pub/Sub for Cloud, In-memory for Standalone).
  - Ensure strict tenant isolation for all mesh broadcasts.
  - Integrate mesh broadcasts to trigger state transitions in the centralized state machine (`src/server/orchestration/statemachine_v2.rs`).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
