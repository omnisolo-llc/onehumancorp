issue_title: "Implement Job Queue Exponential Backoff and DLQ for Proactive Work Triage"
issue_description: |
  # Research Report: Agentic Autonomous System Triage & Feed Operations

  ## Problem Statement
  OneHumanCorp's vision focuses on an "Assistant-first Shell," but current system checks reveal critical gaps in how work is reliably scheduled and fed to the Unified Agent Feed.
  Currently, background tasks (such as `ProactiveAnalysisWorker` and `booking_reengagement` checks) process tasks from `ohc_job_queue`, but fail to handle failure/retry logic consistently. This causes work items to drop and never reach the Owner Feed, meaning real owner/operators like Maya (Baker) or Carlos (Field Service) will silently miss critical leads or actionable insights.
  The system currently lacks a robust Dead Letter Queue (DLQ) and proper exponential backoff implementations required by the "AI Job Queue" section of the architecture.

  ## Research Report & Gap Analysis
  - **Codebase Audits (`src/server/workers/*`, `src/server/db/migrations/*`)**:
    - The `ohc_job_queue` schema defines `max_retries` and `retry_count`, but worker logic drops failed tasks or leaves them hanging without incrementing retry counts or exponential backoff.
    - Some workers forcefully mark failed tasks as `COMPLETED` when exceptions happen, skipping DLQ entirely.
  - **Market Landscape**:
    - High-scale competitors (e.g. Shopify Sidekick, Wix AI) utilize robust, reliable background queues ensuring zero dropped actions. If Maya gets an inquiry, she needs 100% guarantee it shows in her Triage feed.
    - An unreliable queue directly undermines the "Owner Clarity" and "AI Does Useful Work" values.

  ## Design Doc
  - **Architecture**:
    ```mermaid
    graph TD;
      A[Event] --> B[ohc_job_queue (PENDING)];
      B --> C{Worker Process};
      C -->|Success| D[ohc_job_queue (COMPLETED)];
      C -->|Failure & retries < max| E[ohc_job_queue (PENDING) - Inc retry_count, calc next_retry_at];
      C -->|Failure & retries = max| F[ohc_job_queue (FAILED) - Move to DLQ mechanism];
      D --> G[Agent Feed (Triage)];
    ```
  - **Mobile UX Flow**:
    - No changes to direct UI screens on the 375px mobile client. The benefit is felt entirely through *consistent* rendering of cards in the Unified Agent Feed rather than dropping cards.
  - **AI Agent Integration Points**:
    - The `proactive_context_analysis` job logic should have standardized robust trait-based processing. When the LLM processing fails (e.g., API timeout), the job is reliably retried instead of dropped.

  ## Implementation Prompt
  **Goal:** Refactor the backend job workers (like `ProactiveAnalysisWorker`) to strictly adhere to the exponential backoff and maximum retry requirements for the `ohc_job_queue`.
  - Ensure any job that errors out has its `retry_count` incremented and `next_retry_at` updated using an exponential backoff formula.
  - Once `retry_count` reaches `max_retries`, mark the job state as `FAILED` (functioning as an in-place dead-letter queue).
  - Write robust unit tests verifying the retry intervals and state transitions.

  **Priority:** P0 (Critical for data consistency)
  **Estimated Scope:** Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
