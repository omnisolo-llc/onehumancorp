issue_title: "Implement High-Performance Agentic Background Job Queue"
issue_description: |
  # Architecture: High-Performance Agentic Background Job Queue

  ## Problem Statement
  The OHC Hybrid Agentic OS requires a robust, high-performance background job queue to reliably execute asynchronous tasks. Non-technical business owners expect instantaneous UI responses, but AI agent workflows (like the Customer Success "Ambassador" drafting a reply, or the Operations Agent synchronizing inventory) take time. Currently, the lack of a formalized, highly scalable queueing system with persistence, retries, and dead-letter queues limits the platform's ability to scale and guarantees job execution, leading to potential data inconsistencies and "lost" tasks.

  ## Research Report
  - **Market Baseline (Shopify/Stripe):** Leading platforms rely on distributed, durable job queues (e.g., Sidekiq, Celery) to ensure eventual consistency. Stripe extensively uses PostgreSQL-backed queues for transactional guarantees alongside business data.
  - **Current OHC Constraints:** The Go backend utilizes simple goroutines or basic channels which are not durable across server restarts and do not support complex multi-tenant isolation safely.
  - **Architectural Proposal:** Implement a PostgreSQL-backed job queue leveraging the `SKIP LOCKED` pattern. This provides ACID transactional guarantees, seamlessly integrates with our existing multi-tenant data model (enforcing Row Level Security), and avoids the operational complexity of introducing a new infrastructure component (like Kafka or RabbitMQ).

  ## Design Doc
  ### High-Level Architecture
  - **Data Model:** A centralized state store that tracks job payloads, execution status, tenant isolation keys, and retry counters. Strict Row Level Security policies must apply based on the tenant.
  - **Execution Workers:** A pool of background workers that continuously poll the queue safely without lock contention across multiple replicas. When a job is picked up, it is locked, its retry attempts incremented, and its status set to processing.

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Client (375px)
          App[OHC Mobile App] --> API[OHC API Gateway];
      end

      subgraph OHC Backend
          API --> ActionController[Action Controller];
          ActionController --> Queue[(Job Queue State Store)];
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
  **Objective:** Architect and implement a PostgreSQL-backed Job Queue for the backend.
  **CUJ & Acceptance Criteria:**
  1. Define the queue state schema with strict multi-tenant Row Level Security.
  2. Implement the mechanism to enqueue and dequeue jobs safely with high concurrency, avoiding deadlocks.
  3. Implement a worker pool that processes jobs, handles exponential backoff for retries, and moves failed jobs to a dead-letter state after reaching a maximum attempt threshold.
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
