issue_title: "[Architecture] High-Performance Agentic Background Job Queue"
issue_description: |
  # Research Report: High-Performance Agentic Background Job Queue

  ## Problem Statement
  The OHC Hybrid Agentic OS requires a robust, high-performance background job queue to reliably execute asynchronous tasks such as sending emails, syncing inventory, and executing AI agent workflows. Currently, the lack of a formalized queueing system leads to dropped tasks, lack of retry mechanisms, and poor observability.

  ## Research & Findings
  - **Market Analysis:** Established platforms like Shopify and Stripe rely heavily on distributed job queues to ensure eventual consistency and reliability.
  - **Proposed Solution:** Implement a PostgreSQL-backed job queue leveraging the `SKIP LOCKED` pattern. This provides transactional guarantees, seamlessly integrates with our existing multi-tenant data model, and avoids the complexity of introducing a new infrastructure component (like RabbitMQ or Kafka). Redis can be used for pub/sub notifications to wake up workers.

  ## Architectural Design

  ### Data Model
  A new table `agent_jobs` will track background tasks.

  ### Worker Architecture
  - **Producers:** Any part of the application can enqueue a job by inserting a row into `agent_jobs`.
  - **Consumers (Workers):** A pool of Go async workers continuously polls the table using the `FOR UPDATE SKIP LOCKED` pattern.
  - **Exponential Backoff:** Failed jobs are tracked and retried using exponential backoff before being sent to a Dead Letter Queue.

  ### Implementation Plan
  1. Create the `agent_jobs` database migration.
  2. Implement the `JobQueue` repository in `src/server/db.go`.
  3. Create the background worker loop.
  4. Integrate the queue with the AI Orchestrator.

  **Note:** Build verification was skipped during this research phase due to environment issues (missing `bazelisk` binary and external `cncf/xds` network failure), so a maintainer MUST run the test suite manually before merging any implementation of this design.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
