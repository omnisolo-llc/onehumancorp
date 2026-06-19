issue_title: "OneHumanCorp Architecture Design: OHC-HA Work Triage & Async Execution"
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
issue_description: |
  # OHC-HA Work Triage & Async Execution Architecture

  ## Problem Statement

  Small business owners and operators (our core personas like Maya the baker, Carlos the handyman, and Fatima the food cart operator) are currently forced to manage incoming work across fragmented channels: Instagram DMs, SMS, WhatsApp, emails, and web forms.

  When demand spikes or an order is complex, the cognitive load required to triage these inputs, prioritize them, draft context-aware responses, and safely dispatch background work (like calendar updates or quote generation) prevents them from actually running their business. Existing tools either offer basic unified inboxes that require manual human replies, or complex developer-centric agent workflows that are impenetrable to non-technical users.

  **The gap**: OHC lacks a unified, multi-tenant capable, reliable architecture for ingesting cross-channel inputs ("Work Intake"), organizing them into an actionable, prioritized daily feed ("Work Triage"), and safely executing the resulting agentic background tasks (e.g. drafting quotes, generating calendar events, or summarizing trends) in an invisible, async manner that does not block the mobile UI.

  ## Research Report

  - **Competitor Analysis**:
    - **Tencent Workbuddy / WeCom**: Excels at embedding workflows deeply into chat. They use high-performance background queues to handle heavy processing, leaving the chat UI fast.
    - **Shopify Inbox & Sidekick**: Aggregates customer chats and applies basic AI for reply suggestions. However, its background action capabilities (like updating inventory based on a chat) are currently limited and heavily siloed from true "task management".
    - **Wix / GoDaddy / Squarespace**: Offer unified inboxes, but rely almost entirely on synchronous human operation. AI is treated as a "generate text" button, not an autonomous agent that acts on the owner's behalf to update system state.
  - **Internal Audit**:
    - The current codebase contains experimental agent runners (e.g., `src/agents/builtin/worker.rs`, `openhands_runner.rs`) and some workflow concepts (`docs/superpowers/specs/2026-06-07-jarvis-workbuddy-parity-design.md`).
    - There is a lack of a robust, `SKIP LOCKED` based PostgreSQL queue specifically designed for multi-tenant background agent execution and reliable webhook ingestion.
    - We need a reliable ingestion and execution pipeline to power the "Work Triage" capability outlined in the product vision.

  ## Design Doc

  ### High-Level Architecture

  We will implement a resilient, database-backed Async Job & Work Triage Pipeline built on PostgreSQL and Go. This pipeline will ingest events from various channels, normalize them into a `WorkItem` entity, and enqueue them for asynchronous agent processing.

  #### 1. Architecture Diagram (Mermaid)

  ```mermaid
  graph TD
      subgraph Ingestion Layer
          A[External Webhooks] -->|Stripe, IG, Email| B(API Gateway REST)
          C[Mobile App / PWA] -->|gRPC/REST| B
      end

      subgraph Data & Queue Layer
          B -->|Insert WorkItem| D[(PostgreSQL)]
          B -->|Insert Job| E[(Job Queue Table)]
          D -.->|Tenant ID RLS| D
          E -.->|SKIP LOCKED| E
      end

      subgraph Execution Layer
          F[Go Async Workers] -->|Dequeue Job| E
          F -->|Acquire Lock| G[Redis Redlock]
          F -->|Invoke LLM / Agents| H[Gemini / AI Dept]
          F -->|Update Status| E
          F -->|Mutate State / Draft Reply| D
      end

      subgraph Frontend Delivery
          D -->|Poll / SSE| C
      end
  ```

  #### 2. Mobile UX Flow (375px First)

  1. **The Triage Feed (Home Screen)**: The owner opens the app. The primary view is a unified, prioritized list of `WorkItem` cards. Each card represents an ingested event (e.g., "New Custom Cake Request from Sarah via IG").
  2. **Agent Recommendations**: Below the event details on the card, an "Agent Suggestion" block appears (e.g., "Drafting response with $50 deposit link...").
  3. **One-Tap Action**: The owner reviews the AI-generated draft or proposed action and taps a large, thumb-friendly "Approve & Send" or "Modify" button.
  4. **Background Execution**: Upon approval, the UI immediately shows a success state and returns to the feed. The actual sending, database updating, and downstream integrations happen asynchronously via the job queue, completely invisible to the user.

  #### 3. AI Agent Integration Points

  - **The Intake Agent**: Triggered immediately upon new webhook reception. Normalizes raw payload data (e.g., extracting "Sarah", "Vegan Cake", "Next Tuesday") into structured fields on the `WorkItem`.
  - **The Customer Assistant**: Triggered as a secondary job. Reads the structured `WorkItem` and tenant context to draft a personalized reply or propose an action (e.g., generating a quote).
  - **The Operations Assistant**: Triggered if the request involves scheduling or inventory. Checks availability and drafts calendar holds.

  #### 4. Key Design Decisions

  - **PostgreSQL `SKIP LOCKED` for Queueing**: We choose Postgres over a dedicated message broker (like RabbitMQ) to reduce operational complexity and ensure transactional consistency between business data updates and job state changes within the same database transaction.
  - **Row-Level Security (RLS)**: Every table (`work_items`, `jobs`) MUST have a `tenant_id` and utilize Postgres RLS to guarantee strict data isolation between owners.
  - **Idempotency**: All webhook ingestion and job execution paths must be designed to be idempotent to handle retries gracefully without duplicating orders or messages.

  ## Implementation Prompt

  **Role**: Expert Go/PostgreSQL Backend Engineer
  **Task**: Implement the core Work Triage ingestion API and `SKIP LOCKED` job queue architecture for OneHumanCorp.

  **User Journey (CUJ)**:
  As a system operator handling traffic for Maya the baker, when an external webhook (e.g., an Instagram DM payload) hits the ingestion endpoint, the system must securely store the raw data, normalize it into a multi-tenant `WorkItem` record, and reliably enqueue an asynchronous processing job for the AI agents to draft a response, without blocking the API response to the external provider.

  **Acceptance Criteria**:
  1.  **Database Migration**: Create robust PostgreSQL schemas for `work_items` and `async_jobs`. Both MUST include `tenant_id` and enforce Row-Level Security (RLS). The `async_jobs` table must support state tracking (pending, processing, completed, failed) and retry counts.
  2.  **Ingestion API**: Build a secure REST endpoint (Go) to receive generic webhook payloads. It must authenticate the source (e.g., via signature verification or predefined tokens), extract necessary routing info, and insert the record into `work_items`.
  3.  **Job Enqueueing**: Within the same transaction as step 2, enqueue a new job in the `async_jobs` table.
  4.  **Worker Implementation**: Implement a resilient Go worker pool that continuously polls the `async_jobs` table using the `SELECT ... FOR UPDATE SKIP LOCKED` pattern.
  5.  **Execution Stub**: The worker should dequeue jobs, execute a dummy "Agent Processing" function (simulating the AI delay), update the job status to 'completed', and update the `WorkItem` status.
  6.  **Testing**:
      - Write comprehensive unit tests for the worker logic, ensuring `SKIP LOCKED` correctly handles concurrent workers without duplicate processing.
      - Write integration tests verifying the full flow from API ingestion to job completion.

  ## Priority & Scope
  **Priority**: P0 (Critical - Foundational Architecture)
  **Estimated Scope**: Large
