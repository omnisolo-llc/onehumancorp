issue_title: "Implement High-Performance AI Job Queue & Omni-Channel Work Triage Pipeline"
issue_description: |
  ## Product-use Evidence (Phase -1 & Phase 0)
  **Persona:** Maya (Baker)
  **Observed UI Flow & Gap:** During simulated usage in the live Docker Compose stack on a 375px viewport, I acted as Maya receiving a sudden burst of custom cake inquiries via web forms and simulated Instagram DMs. Currently, the product attempts to process intent and generate agent responses synchronously. When multiple requests arrive simultaneously, the synchronous LLM calls block the main thread, resulting in visible UI hangs, network timeout errors (504s), and dropped customer leads. There is no persistent mechanism to safely queue these inbound intents, handle LLM provider rate limits, or show a resilient "Processing" state in the owner's feed.
  **Business Impact:** If a business owner loses a custom order because the AI agent timed out during a traffic spike, trust in the product is permanently destroyed. The Work Triage capability fundamentally requires a robust, async background queue.

  ## Problem Statement
  As OHC ingests demand from multiple asynchronous channels (Instagram DMs, customer emails, checkout events, and forms), processing these requests synchronously via LLM APIs is structurally fragile. We lack a persistent, transactional AI Job Queue. Without it, bursty traffic causes rate-limit failures, dropped messages, and incomplete tasks. Maya needs an invisible, resilient intake pipeline that guarantees every inquiry is safely captured, analyzed by the Work Triage agent, and reliably surfaced as a structured action item in her daily feed—even if she is offline or the LLM provider experiences latency.

  ## Research Report (Track 1)
  - **Competitor Analysis:**
    - **Shopify:** Utilizes a highly robust background job system (historically Resque/Sidekiq, now custom built) for all webhooks and heavy operational tasks.
    - **Stripe:** Guarantees webhook delivery via an internal event bus with exponential backoff and idempotency keys, ensuring zero dropped payments.
    - **Zendesk:** Normalizes omni-channel tickets (social, email, chat) into a unified asynchronous event stream before routing them to agents.
  - **OHC Gap:** OHC's "Work Triage" agent capability requires this same level of enterprise resilience, but packaged invisibly for SMBs. We must implement a PostgreSQL `SKIP LOCKED` job queue pattern. This avoids the operational overhead of managing a separate queue cluster (like Kafka or heavy RabbitMQ) while providing ACID-compliant task orchestration that perfectly integrates with our row-level multi-tenant isolation.

  ## Design Doc (Track 2 & Track 3)

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant External as Webhooks / DMs / Forms
      participant API as OHC API Layer
      participant DB as PostgreSQL (ai_job_queue)
      participant Worker as Triage Agent Worker
      participant Feed as Owner Mobile Feed (UI)

      External->>API: Inbound Event (e.g. New Cake Inquiry)
      API->>DB: INSERT INTO ai_job_queue (status='pending')
      API-->>External: 202 Accepted (Idempotent)
      loop Every interval
          Worker->>DB: SELECT FOR UPDATE SKIP LOCKED
          DB-->>Worker: Return Job
          Worker->>Worker: LLM Intent Classification
          alt Success
              Worker->>DB: Update Job status='completed', Insert Actionable Task
              Worker->>Feed: Emit WebSocket Update
          else Failure / Rate Limit
              Worker->>DB: Update Job status='failed', retry_count++, Exponential Backoff
          end
      end
  ```

  ### Mobile UX Flow (375px Viewport)
  1. **Triage Feed:** The owner opens the app. The primary screen is the "Today's Work" feed.
  2. **Pending States:** If the AI is actively processing a burst of DMs, a non-intrusive translucent glass chip appears at the top: `✨ Sorting 3 new inquiries...`
  3. **Actionable Cards:** Once the job completes, it appears as a clean card: "Maya, 1 new vegan cake request from Alex. Agent drafted a reply. [Review & Send]".
  4. **Error Recovery:** If a job hits the Dead Letter Queue, it surfaces as a manual fallback: "Could not auto-reply to 1 message. [Tap to reply manually]". Touch targets for these actions are strictly >= 44x44px.

  ### AI Agent Integration Points
  - **The Work Triage Agent:** Runs as a continuous background worker polling the `ai_job_queue`. It consumes raw `IntakeEvent` payloads, analyzes the intent, and emits structured `ActionableTask` records to the Shared Task List.
  - **Context Isolation:** Every job payload strictly enforces `tenant_id` to guarantee zero-trust multi-tenancy.

  ### Key Design Decisions
  1. **PostgreSQL SKIP LOCKED over Redis/Kafka:** Given our K8s multi-tenant architecture, managing a single transactional store for both domain data and job queue guarantees referential integrity. `SELECT ... FOR UPDATE SKIP LOCKED` allows highly concurrent, race-condition-free dequeueing by horizontally scaled worker pods.
  2. **Dead Letter Queue (DLQ):** Jobs that fail > 5 times (e.g., persistent LLM moderation block) move to a DLQ status, ensuring they do not poison the queue and block other tasks.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the backend infrastructure for the AI Job Queue using the PostgreSQL `SKIP LOCKED` pattern.
  1. **Database:** Create migrations for an `ai_jobs` table. It must include `tenant_id`, `job_type`, `payload` (JSONB), `status` (pending, processing, completed, failed, dead_letter), `retry_count`, `next_retry_at`, and `locked_at`.
  2. **Worker Logic:** Implement a resilient worker loop in Rust (or Go, per the specific backend service) that polls this table using `SELECT ... FOR UPDATE SKIP LOCKED`.
  3. **Resilience:** Add exponential backoff logic for failed jobs and DLQ promotion after a maximum retry threshold.
  4. **Acceptance Criteria:**
     - 100% unit test coverage for the dequeue and retry logic.
     - Integration tests proving that multiple concurrent workers do not claim the same job.
     - A Playwright E2E test simulating a burst of inbound requests and validating that the UI correctly reflects the eventual processing via the Work Triage feed.

  ## Priority
  `P0`

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
