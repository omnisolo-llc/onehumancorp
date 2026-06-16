issue_title: "[Architecture] High-Performance Agentic Background Job Queue"
issue_description: |
  # High-Performance Agentic Background Job Queue

  ## Problem Statement
  The OHC Hybrid Agentic OS requires a robust, high-performance background job queue to reliably execute asynchronous tasks. Non-technical business owners expect instantaneous UI responses, but AI agent workflows (like the Customer Success "Ambassador" drafting a reply, or the Operations Agent synchronizing inventory) take time. Currently, the lack of a formalized, highly scalable queueing system with persistence, retries, and dead-letter queues limits the platform's ability to scale and guarantees job execution, leading to potential data inconsistencies and "lost" tasks.

  ## Research Report
  - **Market Baseline (Shopify/Stripe):** Leading platforms rely on distributed, durable job queues (e.g., Sidekiq, Celery) to ensure eventual consistency. Stripe extensively uses PostgreSQL-backed queues for transactional guarantees alongside business data.
  - **Current OHC Constraints:** The Go backend utilizes simple goroutines or basic channels which are not durable across server restarts and do not support complex multi-tenant isolation safely.
  - **Architectural Proposal:** Implement a PostgreSQL-backed job queue leveraging the `SKIP LOCKED` pattern. This provides ACID transactional guarantees, seamlessly integrates with our existing multi-tenant data model (enforcing Row Level Security), and avoids the operational complexity of introducing a new infrastructure component (like Kafka or RabbitMQ).

  ## Design Doc
  ### High-Level Architecture
  - **Data Model (`agent_jobs`):** A centralized table storing job payloads, execution status, tenant isolation keys, and retry counters.
    ```sql
    CREATE TABLE agent_jobs (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        tenant_id VARCHAR NOT NULL,
        job_type VARCHAR NOT NULL,
        payload JSONB NOT NULL,
        status VARCHAR NOT NULL DEFAULT 'pending',
        attempts INT NOT NULL DEFAULT 0,
        max_attempts INT NOT NULL DEFAULT 3,
        run_at TIMESTAMPTZ NOT NULL DEFAULT now(),
        locked_at TIMESTAMPTZ,
        locked_by VARCHAR
    );
    CREATE INDEX idx_agent_jobs_pickup ON agent_jobs (status, run_at) WHERE status = 'pending';
    ```
  - **Execution Workers:** A pool of Go workers continuously polls the table using:
    ```sql
    UPDATE agent_jobs
    SET status = 'processing', locked_at = now(), locked_by = $1, attempts = attempts + 1
    WHERE id = (
        SELECT id FROM agent_jobs
        WHERE status = 'pending' AND run_at <= now()
        ORDER BY run_at ASC
        FOR UPDATE SKIP LOCKED
        LIMIT 1
    ) RETURNING *;
    ```
  - **Agent Integration:** AI departments (Marketing, Sales, Operations) publish intent payloads to this queue rather than executing long-running LLM calls synchronously.

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Client (375px)
          App[OHC Mobile App] --> API[OHC API Gateway];
      end

      subgraph OHC Backend
          API --> ActionController[Action Controller];
          ActionController --> Queue[(Postgres agent_jobs)];
      end

      subgraph Worker Nodes
          Queue --> Worker1[Agent Worker Pool 1];
          Queue --> Worker2[Agent Worker Pool N];
          Worker1 --> OpsAgent[Operations Agent];
          Worker2 --> MarketingAgent[Marketing Agent];
      end

      subgraph External Systems
          OpsAgent --> LLM[LLM Provider API];
          MarketingAgent --> Email[Email Service];
      end
  ```

  ### Mobile-First & UX Impact
  - **Optimistic UI:** When Carlos the Handyman approves a quote, the UI updates instantly. The heavy lifting (email generation, calendar syncing, PDF creation) is pushed to the queue.
  - **Visuals:** Uses skeleton loading states and subtle push notifications upon job completion instead of blocking loaders.

  ## Implementation Prompt
  **Objective:** Architect and implement a PostgreSQL-backed Job Queue for the Go backend.
  **CUJ & Acceptance Criteria:**
  1. Define the `agent_jobs` PostgreSQL schema migration with strict multi-tenant Row Level Security.
  2. Implement the `JobQueue` repository to `enqueue` and `dequeue` jobs using the `FOR UPDATE SKIP LOCKED` pattern.
  3. Implement a worker pool that processes jobs, handles exponential backoff for retries, and moves failed jobs to a dead-letter state after max attempts.
  4. Provide comprehensive unit and integration tests verifying transactional safety and multi-worker contention handling.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
