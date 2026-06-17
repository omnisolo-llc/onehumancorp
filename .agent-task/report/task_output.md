issue_title: "[Platform] Implement Global Event Ingestion Pipeline for Agent Feed"
issue_description: |
  ## Problem Statement
  The OHC platform's core differentiator is "invisible AI automation"—proactively pushing critical updates, drafted communications, and suggested actions directly to the business owner. Currently, OHC lacks a unified Event Ingestion Pipeline capable of capturing events (webhooks, internal state changes) and feeding them to the AI Intent & Context Resolution layer. Without this pipeline, agents cannot proactively assist personas like Maya or Carlos with real-time tasks like responding to DMs or recovering abandoned carts.

  ## Research Report
  - **Market Context**: Traditional platforms (Shopify, Wix) rely on third-party apps and manual configuration (e.g., setting up Klaviyo triggers). OHC’s "Zero-Setup" vision requires a native, event-driven architecture where agents observe a central stream and act autonomously.
  - **Core Pain Point Addressed**: Small business owners suffer from tool fatigue and missed opportunities (abandoned carts, delayed DM replies). A unified event pipeline is the foundational prerequisite for the Agent Feed, enabling the `Customer Success Agent` or `Operations Agent` to trigger automatically.
  - **Reference**: `agent_feed_deep_dive.md` and `ai_agentic_workflows_research.md` clearly specify the need for an Event Ingestion Pipeline as the first step in the Agent Feed architecture.

  ## Design Doc
  ### Architecture Diagram (High-Level)
  ```mermaid
  graph TD;
      A[External Webhooks e.g. Stripe, IG] -->|HTTP POST| B(Event Gateway API);
      C[Internal State Changes e.g. New Order] -->|gRPC/Internal| B;
      B -->|Publish| D{Central Message Bus Redis/Kafka};
      D -->|Subscribe| E[Agent Orchestrator Worker];
      E --> F[AI Intent & Context Resolution];
      F --> G[Action Card Generation];
      G --> H[User Mobile Feed];
  ```

  ### System Components
  1. **Event Gateway**: A robust HTTP/REST and internal endpoint to ingest events. Must handle high throughput and implement basic validation/authentication (e.g., webhook signature verification).
  2. **Message Bus / Queue**: Utilization of the existing Valkey (Redis) infrastructure (or PostgreSQL `SKIP LOCKED` pattern as defined in the architecture) to buffer and distribute events reliably.
  3. **Event Schema**: A standardized JSON/Protobuf schema defining `event_type`, `tenant_id`, `source`, `payload`, and `timestamp`.

  ### AI Agent Integration Points
  The pipeline acts as the sensory input for the AI agents. The Agent Orchestrator will listen to this bus, pull events, hydrate them with context via RAG (querying the specific `tenant_id` data), and trigger the appropriate LLM prompt to generate an Action Card.

  ### Mobile UX Impact
  This is a backend infrastructural task, but it directly enables the primary mobile UX: The Agent Feed. Reliable, low-latency ingestion ensures Maya sees the "Drafted Reply to Instagram DM" card within seconds of the customer sending it.

  ## Implementation Prompt
  **Objective**: Design and implement the backend Event Ingestion Pipeline that captures external/internal events and queues them for AI processing.
  **CUJ / Acceptance Criteria**:
  1. A backend service/endpoint exists to accept generic business events.
  2. Events are successfully validated, categorized, and placed onto a reliable queue (e.g., Redis or Postgres-based).
  3. A worker skeleton exists that can dequeue these events and log them, preparing for the next phase of AI Intent Resolution.
  4. The system respects strict `tenant_id` isolation to prevent cross-contamination of events.
  5. Include comprehensive unit and E2E tests proving an event can be published and successfully consumed by the worker.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
