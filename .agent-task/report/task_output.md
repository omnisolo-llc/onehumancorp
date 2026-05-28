issue_title: "Implement Invisible High-Performance Background Agent Job Queue"
issue_description: |
  # [Architecture] Invisible High-Performance Background Agent Queue

  ## Problem Statement

  For small business owners like **Maya** (a baker who receives dozens of Instagram DMs overnight asking "do you do vegan cakes?") and **Priya** (a boutique owner managing in-store inventory and an online storefront), staying on top of customer interactions and operational tasks is overwhelming. They don't have the time or technical expertise to manage manual data entry, manual inventory syncs, or instantly responding to customer inquiries 24/7.

  When Maya goes to sleep, she needs to know her business is still running. If 50 people DM her, the system must reliably capture, process, and respond to each inquiry without dropping a single message or double-booking a custom cake slot. Priya needs her in-store sales to instantly trigger inventory updates online and reorder suggestions without a loading screen or manual refresh.

  They need an invisible, zero-latency system that handles massive concurrency behind the scenes, ensuring that AI agents can reliably process heavy tasks (like analyzing a DM, checking inventory, drafting a response, and securely updating the database) without impacting the smooth, fast experience on their mobile devices. They shouldn't ever see a spinner or a "processing" screen.

  ## Research Report

  Our audit of the current OneHumanCorp (OHC) platform architecture reveals a significant gap: we lack a unified, multi-tenant, high-performance background job queue explicitly designed for asynchronous AI agent operations. While our synchronous API handles standard CRUD operations well, AI agent tasks (LLM generation, context retrieval, multi-step orchestration) are inherently high-latency and unpredictable.

  **Competitive Analysis:**
  - **Shopify:** Utilizes robust background processing (via Sidekiq/Kafka) for inventory syncs, webhook deliveries, and app integrations, ensuring the storefront remains hyper-fast. However, their architecture is primarily designed for deterministic tasks, not stateful, long-running AI agent workflows.
  - **Wix:** Employs event-driven architectures for their Velo platform, allowing developers to write background jobs. But this requires coding, defeating our "no-code" mandate.
  - **Stripe:** Masters the art of reliable, idempotent webhooks and background processing (e.g., Stripe Billing retries). They guarantee at-least-once delivery, which is critical for financial transactions and must be applied to our AI agent actions.

  **Findings & Opportunities:**
  To dominate the market, OHC must introduce an *Agentic Job Queue*. This queue must seamlessly handle task persistence, state transitions, automatic retries with exponential backoff, and strict multi-tenant isolation. When an event occurs (e.g., a new Instagram DM), it should instantly acknowledge receipt to the user (sub-100ms) and enqueue an AI agent task for processing. This ensures the mobile UI remains buttery smooth while the heavy lifting happens invisibly.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ EVENT_TRIGGER : "generates"
      EVENT_TRIGGER ||--o{ AGENT_JOB : "enqueues"
      AGENT_JOB {
          string job_id PK
          string tenant_id FK
          string agent_department
          string status "Pending, Processing, Completed, Failed"
          json payload
          int retry_count
          timestamp created_at
      }
      AGENT_JOB ||--|| JOB_RESULT : "produces"

      WORKER_POOL ||--o{ AGENT_JOB : "consumes & processes"
      WORKER_POOL }|--|| LLM_PROVIDER : "calls"
      WORKER_POOL }|--|| TENANT_DATABASE : "reads/writes isolated data"
  ```

  ```mermaid
  sequenceDiagram
      participant MobileUI as Maya's Mobile App
      participant EdgeGateway as Edge API / Webhook
      participant JobQueue as Invisible Job Queue
      participant AIWorker as Operations AI Agent
      participant DB as Multi-tenant Database

      MobileUI->>EdgeGateway: User Action / Webhook (e.g., New DM)
      EdgeGateway->>JobQueue: Enqueue Agent Job (Tenant Context)
      JobQueue-->>EdgeGateway: Job ID (Ack)
      EdgeGateway-->>MobileUI: 200 OK (Instant Response)

      JobQueue->>AIWorker: Dispatch Job (based on load & priority)
      AIWorker->>DB: Fetch Context (Strict Tenant Isolation)
      AIWorker->>AIWorker: Process LLM Task (Generate Reply)
      AIWorker->>DB: Persist Action / Update State
      AIWorker->>JobQueue: Mark Job Completed
      JobQueue->>MobileUI: Push Notification (Optional: "Reply sent to 5 customers")
  ```

  ### Mobile UX Flow & UI Wireframes (375px Mobile-First)

  **The "Invisible" Experience (What the user *doesn't* see):**
  - When Maya receives an influx of DMs, her app does not freeze. The system instantly accepts the webhooks.
  - There are no global loading spinners.

  **The "Activity Feed" (What the user *does* see):**
  - **Screen:** `Dashboard > AI Activity`
  - **Layout:** Clean, macOS Translucent Glass styling, utilizing Ubiquiti-style modular cards.
  - **Header:** "AI Assistants at Work" (Sticky top).
  - **Cards:**
    - Each card represents a summarized batch of completed agent jobs.
    - *Example Card:* "Responded to 12 Instagram DMs regarding custom cakes. [View Log]"
    - *Example Card:* "Inventory for 'Red Velvet' synced across online and POS. [Details]"
  - **Interaction:** Tapping a card slides in a detailed log. It's read-only, ensuring the user feels informed but not burdened with management. All technical terms (retry queues, latencies) are hidden behind an "Advanced Settings" toggle.

  ### AI Agent Integration Points
  - **Customer Service (CS) Department:** Listens to inbound message queues, retrieves customer history, generates localized responses, and queues outbound message jobs.
  - **Operations Department:** Listens to inventory threshold events, queues reorder suggestion jobs, and orchestrates multi-channel syncs.
  - **Memory Layer:** All job outcomes are fed back into the AutoDream pipeline to update the long-term embedded vector truth for the specific tenant.

  ### Zero Trust & Security
  - **Strict Multi-Tenant Isolation:** Every job in the queue MUST carry an authenticated `tenant_id` (organization ID). Workers processing the queue establish a secure, SPIFFE/SPIRE authenticated connection to the database that strictly enforces Row-Level Security (RLS) based on the `tenant_id`. An agent processing Maya's jobs physically cannot query Priya's data.
  - **Idempotency:** All job processing must be idempotent. If a worker crashes mid-task and a job is retried, it must not result in duplicate actions (e.g., charging a customer twice or sending the same email twice).

  ### Key Design Decisions
  - **Asynchronous First:** Shift all non-critical path AI processing from synchronous API requests to this asynchronous queue to guarantee edge API latency remains under 100ms.
  - **Stateless Workers:** Agent workers must be entirely stateless, pulling all necessary context from the persistent datastore per job, allowing infinite horizontal scaling.

  ## Implementation Prompt

  **Role:** Implementer Agent
  **Task:** Build the core infrastructure for the "Invisible High-Performance Background Agent Job Queue" and its integration with the existing OHC API and AI agent departments.

  **Objective:**
  Provide a scalable, reliable, and strictly multi-tenant background processing system that allows the API to instantly acknowledge requests while heavy AI tasks (LLM generation, multi-step orchestration) are processed asynchronously.

  **User Journey (CUJ):**
  1. An external event occurs (e.g., high volume of webhooks from Instagram) or a business owner performs a bulk action on their mobile device.
  2. The OHC API instantly returns a success response (sub-100ms) and seamlessly enqueues the tasks.
  3. The background workers pick up the tasks, executing the appropriate AI agent protocols while strictly adhering to tenant data isolation.
  4. The system gracefully handles retries for any transient failures (e.g., LLM provider rate limits) without any user intervention.
  5. The business owner sees a clean summary of completed actions in their "AI Activity" feed.

  **Acceptance Criteria:**
  - The queue system supports durable job persistence, prioritization, and exponential backoff for retries.
  - Workers can securely authenticate and impersonate the correct tenant context when accessing the database or other internal services.
  - The system prevents any cross-tenant data leakage during job processing.
  - A clean interface is provided for enqueueing jobs from the edge API.
  - The solution integrates seamlessly with the existing KAIROS orchestration engine and Distributed State Machine.
  - Ensure all technical complexity is abstracted away from any UI components, maintaining the "grandmother test" standard for the mobile experience.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
