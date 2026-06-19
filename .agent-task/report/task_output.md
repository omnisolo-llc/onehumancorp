issue_title: "Multi-Agent Saga Coordination Engine for Reliable Cross-Agent Workflows"
issue_description: |
  ## Target Persona: Nora (Agency Principal)
  Nora handles multi-step processes like project intake, proposal drafting, invoicing, and task assignment. These involve coordination across different AI agent departments (Operations, Sales, Finance).

  ## Problem Statement
  OHC currently has distinct AI capabilities (Triage, Customer, Operations, Sales, Finance, Knowledge). However, there is a gap in orchestrating complex, multi-step business processes that require actions from multiple agents sequentially or in parallel, while guaranteeing system consistency. If the Sales Agent drafts a proposal but the Finance Agent fails to schedule the invoice reminder due to a network error, the system is left in an inconsistent state, causing Nora to manually intervene—breaking the "Invisible AI Automation" promise.

  Small business owners need reliable, hands-off automation. A failure mid-workflow should not drop the ball; it should either retry, compensate (rollback), or escalate gracefully to the owner.

  ## Research Report
  - **Codebase Audit:** OHC currently utilizes background workers (`src/server/workers`), PostgreSQL `SKIP LOCKED` job queues, and some distributed locking (Redis Redlock). However, it lacks a dedicated workflow orchestration engine capable of managing distributed transactions or "Sagas" across its AI agents.
  - **Industry Patterns:** Leading platforms handle long-running, multi-step processes using the Saga Pattern or similar orchestration engines (e.g., Temporal, AWS Step Functions). These systems provide durability, state management, retries, and compensation logic, ensuring that multi-step workflows either complete successfully or fail gracefully.
  - **Competitor Systems Audit:** Enterprise tools like ServiceNow or Salesforce have robust workflow engines. Simpler tools like Zapier provide multi-step zaps but lack the deep, AI-native state management OHC requires. Shopify Flow allows automation but relies on basic triggers/actions rather than intelligent, agent-coordinated Sagas.
  - **The Gap:** OHC needs a lightweight, native Saga Coordination Engine built on top of its existing PostgreSQL queue. This engine will orchestrate handoffs between the Sales, Operations, and Finance agents, ensuring reliable execution of complex journeys (like Nora's project intake).

  ## Design Doc
  ### Data Model & Invariants (PostgreSQL)
  - `SagaExecution`: Tracks the overall multi-step workflow. Fields: `id`, `tenant_id`, `saga_type` (e.g., `ProjectIntake`), `status` (`running`, `completed`, `compensating`, `failed`), `context` (JSONB).
  - `SagaStep`: Tracks individual agent tasks within a Saga. Fields: `id`, `saga_id`, `tenant_id`, `step_name`, `agent_type`, `status`, `retry_count`.
  - **Invariants:** Row-level security on `tenant_id`. Strict state machine transitions (e.g., cannot transition from `completed` to `running`).

  ### Architecture Flow
  1. **Trigger:** An event (e.g., Nora approves a project intake form) initiates a new `SagaExecution` via the orchestrator.
  2. **Step Execution:** The orchestrator dispatches the first `SagaStep` (e.g., "Draft Proposal") to the relevant agent's job queue.
  3. **Handoff:** Upon successful completion, the agent updates the `SagaStep` and yields back to the orchestrator, which then dispatches the next step (e.g., "Schedule Invoice Reminder" to Finance).
  4. **Compensation:** If a step fails persistently, the orchestrator triggers compensation steps (e.g., canceling drafted documents) for previously completed steps to maintain consistency.

  ### AI Agent Coordination
  - The Saga Engine acts as the "Chief of Staff," managing the state and routing tasks to specialized departments (Sales, Finance, Ops) without the agents needing to know about the overarching workflow.

  ### Mobile UX Flow (375px)
  - **Saga Visibility:** Nora's Agent Feed shows a single unified "Project Intake in Progress" card, rather than disjointed updates from different agents. The card displays a progress indicator (e.g., "Drafting Proposal -> Setting up Tasks -> Scheduling Invoice").
  - **Escalation:** If the Saga requires owner input or fails, the card transitions to an actionable state, allowing Nora to unblock it with a single tap.

  ## Implementation Prompt
  **Feature:** Multi-Agent Saga Coordination Engine
  **User Outcome:** Ensure that multi-step agent workflows (like project onboarding) execute reliably without dropping tasks or leaving the system in an inconsistent state.
  **CUJ (Critical User Journey):**
  1. Nora approves a new client project in the mobile app.
  2. The system initiates a `ProjectIntake` Saga.
  3. The Operations Agent creates project tasks. (Step 1)
  4. The Finance Agent attempts to schedule an invoice reminder but fails due to a temporary integration issue. (Step 2)
  5. The Saga Engine automatically retries the Finance Agent step.
  6. Upon success, Nora sees a completed "Project Setup" card in her feed.

  **Next Actions for Engineering:**
  1. Design and implement the PostgreSQL schema for `SagaExecution` and `SagaStep` with strict multi-tenant RLS.
  2. Build the Go-based Saga Orchestrator logic to manage state transitions, dispatch tasks to existing agent queues, and handle retries/compensation.
  3. Integrate the orchestrator with the Agent Feed to provide unified progress cards for the owner on mobile.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []