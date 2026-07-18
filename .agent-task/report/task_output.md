issue_title: "Implement High-Performance AI Job Queue for Background Workflows"
issue_description: |
  ## Research Report: High-Performance AI Job Queue for Background Workflows

  ### 1. Problem Statement
  OHC handles operations that naturally fall into background asynchronous processing, such as sending emails, syncing calendars, preparing quotes, or requesting multi-step actions from AI models. Our target personas—like Maya the baker and Carlos the handyman—rely on these operations happening seamlessly behind the scenes without tying up the main UI thread.

  Currently, OHC lacks a dedicated, reliable, high-performance background job queue capable of coordinating distributed AI agent tasks and managing retry logic, failures, and concurrency efficiently. If Carlos generates 5 quotes and the LLM API throttles him, he shouldn’t lose data or see an unresponsive UI.

  ### 2. Research Findings & Gap Discovery
  **Current State:** Based on the codebase and design docs, the current OHC architecture heavily relies on synchronous API calls to external providers (like Gemini, OpenAI, MiniMax) for agent tasks. While PostgreSQL and Redis exist in the docker-compose stack, there is no robust queue system built out for processing background jobs.

  **Industry Benchmarks:**
  - **Shopify:** Utilizes robust background queues (using Kafka and Resque) to process webhooks and long-running inventory tasks.
  - **Stripe:** Exclusively uses asynchronous events and idempotent queue workers for payment intents and webhooks to guarantee zero dropped messages.
  - **Wix/Squarespace:** Defers complex site generation or data migrations to background processes, alerting the user via notifications once complete.

  **The Gap:** OHC needs a robust, scalable, and resilient AI job queue. This is critical for scaling "Agentic Workflows" where multi-step reasoning models (or long-running LLM calls) require guaranteed execution, automatic retries with exponential backoff, and dead-letter queues (DLQ) for manual inspection of failed jobs.

  ### 3. Design Doc

  #### 3.1 Architecture diagram
  ```mermaid
  erDiagram
      OHCJobs {
          uuid id PK
          string tenant_id FK
          string status "pending, processing, completed, failed"
          jsonb payload
          int retry_count
          timestamp next_run_time
      }
      WorkerPool ||--o{ OHCJobs : processes
      WorkerPool {
          string worker_id
      }
  ```

  Given that PostgreSQL and Redis are already part of the OHC stack, the most robust, dependency-light path is to leverage **PostgreSQL `SKIP LOCKED`** functionality to build an ACID-compliant, reliable job queue.

  **Architecture Overview:**
  - **Job Table (`ohc_jobs`)**: Stores job payload, status (pending, processing, completed, failed), retry count, next run time, and tenant context.
  - **Dequeue Pattern**: Workers fetch jobs using `SELECT ... FOR UPDATE SKIP LOCKED`, ensuring multiple workers don’t process the same job concurrently and jobs are not lost if a worker crashes.
  - **Worker Pool**: A set of Go background goroutines that continuously poll the job table, execute tasks, and update status.
  - **Dead Letter Queue (DLQ)**: Jobs that fail beyond max retries are moved to a `failed` state for review, preventing poison-pill loops.

  #### 3.2 Mobile UX Flow (375px)
  - **Action Trigger**: Maya clicks "Generate Quotes for 5 inquiries".
  - **UI State**: The UI immediately returns to an optimistic state, showing a "Generating..." toast or inline indicator.
  - **Background Processing**: The API enqueues 5 jobs. The Go workers pick them up and call the LLM API.
  - **Completion & Notification**: Once jobs complete, a WebSocket or polling mechanism updates the UI, replacing "Generating..." with the final quotes. A notification bubble appears on the main screen.

  #### 3.3 AI Agent Integration Points
  - `Customer & Relationship Assistant`: Enqueues "Draft Email Reply" jobs.
  - `Operations Assistant`: Enqueues "Sync Booking to Google Calendar" jobs.
  - `Sales & Revenue Assistant`: Enqueues "Generate Proposal PDF" jobs.

  ### 4. Implementation Prompt
  **To the Implementer:**
  Implement a PostgreSQL-backed job queue for background AI and operational tasks in the Go backend.

  **Acceptance Criteria:**
  1.  **Data Schema:** Create the necessary database migrations to add a jobs table (e.g., `ai_job_queue`) with robust multi-tenant isolation.
  2.  **Queue Logic:** Implement enqueue, dequeue (using `FOR UPDATE SKIP LOCKED`), success, and failure (with retry logic and DLQ) operations in Go.
  3.  **Worker Daemon:** Implement a background goroutine worker pool that polls the queue and processes jobs asynchronously.
  4.  **Integration:** Refactor at least one long-running AI capability (e.g., generating a response or a quote) to use the new queue instead of synchronous processing.
  5.  **UI Update:** Ensure the frontend correctly handles the asynchronous nature of the job, displaying appropriate loading states and polling/listening for completion.
  6.  **Tests:** 100% unit test coverage for the queue logic. E2E Playwright tests verifying the background processing flow in the UI. All `bazel test //...` must pass.

  **Important:** Focus on the robustness of the queue and the smooth user experience during background processing. Do not introduce new external dependencies like RabbitMQ or Kafka; utilize the existing PostgreSQL database.

  ### 5. Estimated Scope
  **Medium**

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report, backend, architecture]
assignees: []
