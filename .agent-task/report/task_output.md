issue_title: "Implement High-Performance Distributed Background Job Queue for Agent Automation"
issue_description: |
  # Research Report: High-Performance Distributed Background Job Queue for Agent Automation

  ## Executive Summary
  This report investigates the architectural gaps in OneHumanCorp's (OHC) AI agent automation capabilities, specifically focusing on the reliability and scalability of asynchronous work execution. The objective is to design a high-performance distributed background job queue that ensures reliable agent coordination and background task execution without requiring manual intervention from business owners.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  Leading SaaS platforms and specialized tools (e.g., Shopify's background workers, Stripe's event processing, Temporal) rely on robust distributed task queues to handle asynchronous workloads reliably. Currently, many simpler platforms suffer from dropped tasks, lack of retries, or complex setups that fail small business owners when scale increases or intermittent failures occur.

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus:** Nora (Agency Principal) and Maya (Home Baker) who rely on the AI assistant to draft proposals, sync inventory, and send automated follow-ups.
  - **The Gap:** OHC's current architecture lacks a highly reliable, distributed background job queue with guaranteed at-least-once execution, exponential backoff retries, and dead-letter queue (DLQ) capabilities. If an agent task (like sending a quote or syncing inventory) fails due to a transient API error, it might be lost, causing the owner to lose trust in the AI assistant.

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[API Server / Webhook] -->|Enqueue| B(PostgreSQL Job Table)
      B -->|SKIP LOCKED Dequeue| C{Worker Pool}
      C -->|Process| D[AI Agent / Integration]
      D -- Success --> E[Mark Completed]
      D -- Failure --> F{Retry Limit Reached?}
      F -- No --> G[Schedule Retry w/ Backoff]
      F -- Yes --> H[Move to Dead Letter Queue]
      H --> I[Operations Agent Alert]
  ```

  ### Data Model & System Architecture
  - **Storage:** PostgreSQL table (`ohc_job_queue`) acting as the primary store.
  - **Concurrency:** Utilizing PostgreSQL's `FOR UPDATE SKIP LOCKED` to allow multiple concurrent worker nodes to dequeue jobs without lock contention.
  - **Features:**
    - Payload storage (JSONB)
    - Retry counting and next-run-at timestamps
    - Exponential backoff algorithm for transient failures
    - Dead Letter Queue (DLQ) for permanently failed jobs

  ### AI Agent Integration
  - **Operations Agent:** Monitors the DLQ and alerts the owner (e.g., Nora) with plain-language summaries when a critical background task permanently fails, suggesting a manual fallback or configuration fix.

  ### Mobile & UX Integrity
  - The job queue itself is invisible to the user.
  - The UI will only reflect the *outcomes* (e.g., "Proposal Sent" vs "Failed to Send Proposal - Tap to Retry"). This maintains the 375px mobile-first promise by keeping complexity entirely backend.

  ## 4. Proposed Implementation Steps & Issue Prompt

  **Feature Name:** OHC Distributed Background Job Queue

  **Target Persona:** All Owners (focusing on system reliability)

  **Outcome:** Background tasks (emails, agent inferences, syncs) are executed reliably. Transient failures are automatically retried, and permanent failures are surfaced gracefully without technical jargon.

  **Critical User Journey (CUJ) / Implementation Prompt:**
  As an implementer, build a distributed job queue in Rust (backend) backed by PostgreSQL.
  1. Define the database schema for the queue, including fields for payload, status, retries, and next execution time.
  2. Implement the `enqueue` function to add jobs.
  3. Implement the worker loop using `FOR UPDATE SKIP LOCKED` to fetch pending jobs.
  4. Implement exponential backoff for retries.
  5. Implement routing of permanently failed jobs to a DLQ and trigger a mock Operations Agent alert.
  Ensure comprehensive unit and integration tests are written. Do not expose queue management to the frontend; keep it as a backend primitive.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
