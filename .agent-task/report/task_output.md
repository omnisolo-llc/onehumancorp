issue_title: "Implement the Sub-Agent Orchestration Queue for Seamless Inter-Agent Communication"
issue_description: |
  # Sub-Agent Orchestration Queue for OHC

  ## Problem Statement
  OneHumanCorp (OHC) agents currently operate in isolated silos. When an owner interacts with the system, multiple agent domains (Operations, Finance, Sales, Customer Service) often need to coordinate to fulfill complex workflows (e.g., booking a service and collecting a deposit). Without a centralized queue and state tracking mechanism, agents cannot reliably delegate tasks, pass context, or handle failures gracefully, leading to dropped tasks and incomplete operations for the business owner.

  ## Research Report
  - **Shopify Sidekick:** Highly integrated but primarily relies on synchronous API calls rather than a true distributed queue for cross-domain tasks.
  - **Intercom Fin:** Uses a robust state machine to track conversation resolution but is less focused on back-office operational tasks.
  - **OHC Opportunity:** A PostgreSQL-backed job queue leveraging `SKIP LOCKED` combined with a Redis-backed state machine for distributed locks. This architecture will enable asynchronous, reliable task delegation between specialized OHC agents (e.g., Triage -> Sales -> Operations), ensuring zero dropped tasks even during high load or partial system failures. This "invisible hand" is crucial for delivering a "Zero-to-One" autonomous experience.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      TriageAgent[Work Triage Agent] -->|Enqueue Task| DBQueue[(PostgreSQL Queue)]
      DBQueue -->|Dequeue SKIP LOCKED| WorkerPool[Agent Worker Pool]
      WorkerPool --> SalesAgent[Sales & Revenue Agent]
      WorkerPool --> OpsAgent[Operations Agent]
      SalesAgent -.->|Update State| RedisState[(Redis Distributed Lock)]
      OpsAgent -.->|Update State| RedisState
      WorkerPool -->|Mark Completed/Failed| DBQueue
  ```

  ### Mobile UX Flow
  This feature is primarily backend infrastructure, but its effects are visible in the Assistant Shell:
  1. **Assistant Feed (375px):** Owner sees a grouped "Task Progress" card.
  2. **Status Indicators:** "Drafting proposal..." (Sales Agent active) -> "Checking inventory..." (Ops Agent active) -> "Ready for review" (Task complete).
  3. **Handoff Visibility:** Smooth transitions in the UI without blocking the main thread, updating via Server-Sent Events (SSE) or WebSockets as queue jobs complete.

  ### AI Agent Integration Points
  - **Producer:** The Triage Agent identifies complex requests and breaks them into discrete tasks, pushing them to the queue with appropriate context payloads (JSON).
  - **Consumer:** Domain-specific agents (Sales, Ops, Finance) poll the queue for their designated task types, process them, and return structured output.
  - **Coordinator:** The KAIROS Orchestration engine oversees the queue, managing retries, dead-letter routing, and final state aggregation.

  ### Key Design Decisions
  - **PostgreSQL over RabbitMQ/Kafka:** To minimize infrastructure footprint for self-hosted/standalone deployments, a robust PostgreSQL `SKIP LOCKED` queue is preferred for transactional consistency with core application data.
  - **Idempotency:** All sub-agent task handlers must be idempotent to safely handle retries.

  ## Implementation Prompt
  Implement a robust PostgreSQL-backed job queue tailored for inter-agent task delegation.

  1. Define the core queuing logic using the `SKIP LOCKED` pattern to ensure reliable concurrent dequeue operations without locking contention.
  2. Create a generic worker pool structure capable of executing agent-specific payload handlers.
  3. Integrate error handling, including exponential backoff for retries and a dead-letter queue for persistently failing tasks.
  4. Ensure the system is observable (e.g., via OpenTelemetry spans for enqueue, dequeue, and processing events).

  The solution must be fully testable without requiring external message brokers (like RabbitMQ) to maintain compatibility with OHC's standalone deployment mode.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
