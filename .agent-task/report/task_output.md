issue_title: "Implement Time-Delayed Event Scheduling for Cart Recovery Agent"
issue_description: |
  ## Problem Statement
  Small business owners face significant revenue loss from abandoned shopping carts. While OHC has an Agent architecture, it currently lacks a robust, time-delayed event scheduling mechanism to trigger background actions, such as cart recovery emails, reliably across multi-tenant boundaries. Currently, non-technical owners are forced to stitch together third-party tools (like Klaviyo) for delayed marketing sequences, which introduces extreme friction, costs, and cognitive overload.

  ## Research Report
  Our competitive analysis indicates that abandoned cart recovery is one of the highest ROI features a small business can utilize. Platforms like Shopify require external apps for advanced, multi-stage recovery flows. Our unified `ohc_job_queue` and `agent_feed_items` tables provide immediate queuing, but the platform lacks a dedicated time-series scheduler. We need an architectural enhancement to support delayed execution ("Check this cart in 1 hour") natively within our job processing loop.

  ## Design Doc: Architectural Enhancement for Delayed Agent Triggers

  ### Data Model & Invariants
  The current `ohc_job_queue` table supports `next_retry_at`. We must leverage this or introduce a `scheduled_for` column strictly for delayed execution of agent payloads.

  - **Data Isolation**: All scheduled events MUST enforce RLS based on `tenant_id`.
  - **Job Polling**: Extend the worker loop to poll for jobs where `scheduled_for <= NOW()` and `status = 'PENDING'`, utilizing PostgreSQL `SKIP LOCKED` to prevent duplicate processing.
  - **Entity Boundaries**: Create an `AbandonedCartEvent` payload that the Sales Agent will pick up. The payload should include the cart ID, customer context, and original timestamp.

  ### AI Agent Integration
  When the scheduled event fires, the job runner invokes the `Customer Success Agent`. The agent performs RAG against the cart's items and business tone, generating a customized message (e.g., "We saved your vegan cakes!").

  ### Mobile UX Flow (375px)
  For the business owner, this entire process is zero-setup. The only UI requirement is a "Vitality Dashboard" card on the 375px mobile view that reads:
  **"The Assistant recovered 3 abandoned carts this week, securing $140 in revenue."**
  - No complex rules engine setup.
  - Optional: A notification requiring 1-tap "Approve Draft" if the AI decides a special discount should be offered.

  ## Implementation Prompt (For Implementer Swarm)
  **Objective**: Implement the delayed execution feature in the background job queue and connect it to a mock "Cart Abandoned" event that triggers the `Customer Success Agent`.

  **CUJ for the Test**:
  1. A shopping session is marked "abandoned".
  2. The system schedules a follow-up job for 1 hour in the future.
  3. The background worker picks up the job when the time arrives.
  4. The Customer Success Agent drafts a recovery email.

  **Acceptance Criteria**:
  - Extend the `ohc_job_queue` processing logic to reliably handle delayed jobs.
  - Implement 100% unit test coverage for the scheduling logic.
  - Implement a Playwright E2E test that validates a scheduled event successfully queues an Agent feed item. No mocking of the database or API.
  - Ensure strict multi-tenant isolation via RLS policies.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
