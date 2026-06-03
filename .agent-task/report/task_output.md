issue_title: "High-Performance Agentic Background Job Queue Implementation Report"
issue_description: |
  # Research Report: High-Performance Agentic Background Job Queue

  ## Current State
  The current platform utilizes a basic `sub_agent_queue` implementation found in `src/server/queue.rs`. While functional, the existing table schema lacks specific design parameters like priority handling and exponential backoff mechanisms required for a high-performance agentic queue.

  Multiple migration files touch related concepts:
  - `src/server/db/migrations/015_job_queue_and_ledger.sql` contains `ohc_job_queue` setup, however this looks to be in `src/server/db/migrations` and not fully linked to `queue.rs` functions.
  - `src/server/migrations/060_job_queue_and_ledger.sql` introduces `ohc_job_queue` and `ohc_universal_ledger`.
  - The actual `src/server/queue.rs` hardcodes the use of `sub_agent_queue`.

  ## Gap Analysis
  - The `ohc_job_queue` table defined in migrations exists, but `src/server/queue.rs` is still pointing to `sub_agent_queue` which is poorly typed (e.g. `payload TEXT` instead of `JSONB`) and doesn't fully implement prioritization or proper tenant isolations per `ohc_job_queue`.
  - The architecture specifies `Job` data model and invariants, but `queue.rs` uses different naming and lacks these specific fields properly mapped.
  - The goal is to fully port `sub_agent_queue` over to `ohc_job_queue` structure (or migrate `sub_agent_queue` to align with the design) to meet the new P0 requirement for High-Performance Agentic Background Job Queue.

  ## Recommended Actions
  1. Refactor `src/server/queue.rs` to use `ohc_job_queue` table matching the schema in `015_job_queue_and_ledger.sql` and `060_job_queue_and_ledger.sql`.
  2. Implement proper retry strategies with exponential backoff on failure cases.
  3. Support priority levels during queue polling.
  4. Ensure full tenant isolation and row-level security applies.
  5. Enhance telemetry tracking specific to the new `ohc_job_queue` performance.
  6. Eliminate usage of legacy `sub_agent_queue` once fully transitioned.

  This task lays the foundation for all backend AI automation processes, eliminating current bottlenecks and providing reliable asynchronous execution for the KAIROS Orchestrator.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, backend, performance]
assignees: []
