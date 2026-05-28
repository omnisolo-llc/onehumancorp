issue_title: "High-Performance Background Job Queues for Autonomous Operations"
issue_description: |
  # Title: High-Performance Background Job Queues for Autonomous Operations

  ## Problem Statement
  For OneHumanCorp (OHC) to truly act as an invisible AI operating system for small businesses, our AI agents must execute complex workflows asynchronously without degrading the immediate user experience.
  Consider Maya (the baker, 28): She needs her AI agent to automatically reply to Instagram DMs ("Do you do vegan cakes?") while she sleeps, ensuring she doesn't lose leads. If the background system lags, she loses sales.
  Consider Leo (the music tutor, 22): When a student books a subscription package, Leo needs auto-generated meeting links and calendar invites created in the background instantly.
  Currently, OHC lacks a dedicated, high-scale background job queue optimized for multi-tenant isolation and edge deployment, meaning these critical operations could block the main UI threads or fail silently during high load. Non-technical users cannot debug failed Webhook retries or "job timed out" errors. They need a system that "just works" instantly, every time.

  ## Research Report
  To support global scale and strict multi-tenancy, we evaluated industry-standard approaches to background processing:
  - **Shopify's Background Workers**: Utilizes a deeply integrated, proprietary Sidekiq/Ruby-based queuing system. Excellent for their specific monolith, but relies on a heavy stack that doesn't natively support our Zero-Trust agent mesh or edge computing requirements.
  - **Temporal**: Offers robust distributed state machines, durable execution, and excellent retries. However, running Temporal clusters per tenant or scaling a massive shared cluster introduces significant operational overhead and latency for edge-triggered lightweight agent tasks.
  - **Celery/Redis**: The standard for Python, but lacks native durable workflow guarantees and can suffer from message loss during network partitions without heavy configuration.
  **Conclusion**: OHC requires a custom hybrid approach—a Distributed State Machine powered by a high-throughput, low-latency event mesh (like NATS JetStream) that natively integrates with our AI Agent memory isolation protocols. This ensures that Maya's DM replies and Leo's calendar links are durable, instantly queued at the edge, and executed with strict multi-tenant Zero Trust policies.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Edge & Mobile
          A[Mobile Client / 375px UI] -->|Trigger Action| B(Edge API Gateway)
      end

      subgraph Multi-Tenant Job Queue Mesh
          B --> C{NATS JetStream Mesh}
          C -->|Tenant Route: Maya| D[Queue: Tenant A]
          C -->|Tenant Route: Leo| E[Queue: Tenant B]
      end

      subgraph AI Departments
          D --> F[AI Sales Agent Worker]
          E --> G[AI Ops Agent Worker]

          F -->|Fetch Context| H[(Isolated Tenant A Memory)]
          G -->|Fetch Context| I[(Isolated Tenant B Memory)]
      end

      F --> J[External API - Instagram DM]
      G --> K[External API - Calendar]

      F -.->|Status Update| C
      G -.->|Status Update| C
      C -.->|Push Notification| A
  ```

  ### Mobile UX Flow
  - **Interaction**: Maya enables "Auto-Reply to DMs" in the "Advanced Settings" card on her dashboard.
  - **Background Execution**: Once toggled, the system visually indicates activation with a smooth, translucent glass confirmation toast. No loading spinners block her view.
  - **Visibility**: Background jobs (like a batch of 50 DMs being replied to) appear in a non-intrusive "Activity Center" badge at the top right of the 375px viewport. Maya can tap it to see a clean list of "Agents at Work," but is never required to monitor it.

  ### AI Agent Integration Points & Memory Isolation
  - **Event Triggering**: User actions (booking a lesson, receiving a DM) publish normalized events to the hybrid mesh.
  - **Memory Isolation**: Each background worker must acquire a SPIFFE/SPIRE authenticated short-lived token tied exclusively to the tenant's ID before pulling job context. The worker queries the AI Memory Layer specifically partitioned for that tenant, ensuring Maya's customer data is cryptographically isolated from Leo's student data.
  - **Agent Departments**: Jobs are routed to specific AI departments (e.g., the Marketing Agent handles DMs, the Operations Agent handles scheduling).

  ### Key Design Decisions
  1. **Event-Driven Over Polling**: To achieve strict performance targets, workers will subscribe to event streams rather than polling a database, minimizing latency.
  2. **Strict Multi-Tenancy at the Queue Level**: Job queues are logically partitioned by tenant. A spike in one tenant's traffic (e.g., Priya's boutique going viral) will not starve background workers processing Fatima's food cart orders.
  3. **No-Code Visibility**: The complexity of the queues (retries, dead-letter queues) is entirely abstracted. The UI only shows human-readable states: "Agent is typing..." or "Task Completed."

  ## Implementation Prompt
  **To the Implementer Agent:**
  Implement the high-performance background job queue engine that processes asynchronous tasks (like sending emails, calling external APIs, and triggering AI agents) without blocking the user interface.
  - **User Journey (CUJ)**: Maya receives an Instagram DM. The webhook hits our edge gateway, instantly enqueues a job, and returns a 200 OK. In the background, the AI Sales Agent worker picks up the job, securely accesses Maya's isolated memory, and dispatches the reply.
  - **Acceptance Criteria**:
    - Provide a mechanism to enqueue, process, and retry durable jobs asynchronously.
    - Ensure strict tenant isolation: jobs must only execute within the security context of the tenant that owns them.
    - The system must not prescribe the underlying DB schema but must expose a clean interface for enqueuing tasks from the edge.
    - Ensure the solution maintains low latency and can gracefully handle offline queuing and syncing when connectivity is restored for mobile-first actions.

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
