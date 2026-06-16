issue_title: "[Architecture] Teammate Mesh API and State Machine Integration for KAIROS Hub"
issue_description: |
  # Mission Queue Protocol: KAIROS Hub Architectural Gap Discovery

  ## Problem Statement
  While OneHumanCorp provides multi-agent orchestration via KAIROS, a core architectural gap is the lack of a standardized Hub topology to cleanly isolate inter-departmental agent communications via a "Teammate Mesh". We currently lack a comprehensive mesh layer with real-time broadcast APIs and deep integration for inter-agent delegation and state transitions within our Go backend service. As SMB users (like Maya the baker) scale their operations, they need AI agents across distinct departments (e.g., Marketing and Operations) to securely and reliably sync context.

  ## Research Report
  Our competitive analysis indicates that basic setups in Shopify or Wix do not provide native multi-agent coordination (often relying on disjointed third-party app webhooks). OHC's value proposition relies on treating AI as infrastructure. We have an existing KAIROS foundation, but the Go backend lacks robust hub infrastructure to connect them at scale natively for cross-department handoff. We must build a Go-based teammate mesh that leverages robust data stores (Redis Pub/Sub or Postgres SKIP LOCKED) to allow agent transitions to automatically propagate state changes securely.

  ## Design Doc
  ### Architecture
  *   **Mesh API**: Extend the Teammate Mesh APIs within the core orchestration Go modules to handle robust Pub/Sub with robust retry and backoff logic.
  *   **Isolation**: Utilize OHC's row-level security (`tenant_id`) down to the mesh broadcast level to prevent data leakage between tenants.
  *   **Integration**: Connect the mesh broadcasts directly into the centralized Go state machine for verifiable inter-agent delegation.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Marketing Agent] -->|Teammate Mesh Broadcast| B(Redis PubSub / PG Queue)
      B --> C[Operations Agent]
      A -->|State Transition| D(Go State Machine)
      C -->|Verify Delegation| D
      B -.->|Tenant Context Isolation| E[(PostgreSQL)]
  ```

  ### Mobile UX Flow (375px)
  *   **Persona / Context**: Maya (Home Baker) opens OHC on her 375px phone to check custom orders.
  *   **Browser/Playwright Flow Evaluated**: Navigating from the Assistant Feed to the pending agent tasks view (`/assistant/tasks`).
  *   **Observed UI Gap**: There is currently no indicator showing that the Operations Agent is waiting on the Sales Agent for a custom quote approval. The handoff state is invisible.
  *   **Teammate Sync Indicator Fix**: The mobile interface will feature a translucent Glassmorphism indicator (`.glassmorphism` container with `border-radius: 16px`) reading "The Promoter is briefing The Manager" at the top of the Assistant feed.
  *   **Read-Only Log**: Business owners can tap the indicator to see a localized, readable log of inter-agent delegation to build trust.

  ## AI Agent Integration Notes
  - The integration must rely entirely on standardized TeammateMesh events.
  - The StateMachine transitions must accurately trigger broadcast events that other agents can subscribe to asynchronously.
  - Memory isolation is strictly bound by `organization_id` ensuring a Marketing Agent from Tenant A cannot see broadcasts from Tenant B.

  ## Implementation Prompt
  Implement the Teammate Mesh APIs within the KAIROS Orchestration layer for the Go backend. Set up proper multi-tenant broadcast boundaries based on `tenant_id`. Integrate the mesh with the state machine so that events emitted by one agent's completion transition the state of a dependent agent task securely. Describe WHAT the system should do and WHY, not HOW to build it. Do NOT include SQL DDL, API endpoint lists, function signatures, or specific library choices. Ensure unit test coverage is 100%.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
