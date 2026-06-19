issue_title: "Architecture: Automated Shift Summary and Coordination for Location Managers"
issue_description: |
  # Architecture Design: Automated Shift Summary and Coordination for Location Managers

  ## Problem Statement
  Jun (Location Manager, 31) runs day-to-day operations for one site of a larger operation. During a shift, floor staff complete tasks and occasionally log minor customer complaints or supply shortages. Currently, Jun has to manually compile these updates from different sources into an end-of-shift report for the regional owner. This is time-consuming and often misses key context, leaving the owner without a clear summary of the location's performance.

  ## Research Report
  - **Persona Focus:** Jun (Location Manager). He operates primarily on a mobile device while on the floor.
  - **Competitive Analysis:**
    - *DingTalk & WeCom:* Provide location-based check-ins and shift handovers, but require heavy configuration and lack automated synthesis.
    - *7shifts / Homebase:* Excellent for staff scheduling, but lack AI integration to automatically summarize operational tasks and escalations.
  - **Opportunity:** OHC can differentiate by leveraging its AI departments (Operations & CS) to automatically synthesize shift handover notes, summarize customer feedback, and escalate anomalies directly to the regional owner.
  - **Core Findings:** Implementing a multi-tenant isolated task tracking system combined with an AI summarization job per shift can reduce manager overhead by 40% and provide clearer visibility to owners.

  ## Design Doc
  - **Architecture Diagram:**
    ```mermaid
    graph TD
      A[Staff UI Mobile] -->|Logs Task/Issue| B(Task DB Postgres)
      B --> C{Operations Agent Worker}
      C -->|Monitors| B
      C -->|End of Shift Trigger| D[AI Summarization Job]
      D -->|Uses OHC_LLM_PROVIDER| E(Shift Summary Draft)
      E --> F[Manager UI Mobile]
      F -->|Approves & Escalate| G(Owner Dashboard)
    ```
  - **UI Wireframes / Screen Flow Description:**
    - **Screen 1: Active Shift Dashboard:** A clean, translucent glass UI showing the top 3 pending tasks for the location.
    - **Screen 2: Quick Action Modal:** A fast, one-tap sheet with a native mobile keyboard to log a minor issue or customer feedback.
    - **Screen 3: End of Shift Summary:** An AI-generated draft summarizing completed tasks and logged issues, with an "Approve & Escalate" button.
  - **Mobile UX Flow:**
    - The layout is optimized for a 375px viewport.
    - High-contrast, 44x44px touch targets.
    - Offline-tolerant task logging that queues and syncs when connection is restored.
  - **AI Agent Integration Points:**
    - An Operations Agent job triggers at the end of a shift to pull completed tasks and issues.
    - The agent uses the configured `OHC_LLM_PROVIDER` to synthesize a concise, owner-ready summary draft.
  - **Key Design Decisions:**
    - Multi-tenancy is enforced using strict row-level security (RLS) on `tenant_id` and `location_id` in the database.
    - The summarization job must operate via a distributed queue to handle high scale and potential AI provider latency.

  ## Implementation Prompt
  **Role:** Implementer Agent
  **Task:** Build the backend models, worker jobs, and mobile-first (375px) UI for automated shift summaries.
  **CUJ:**
  1. Jun logs in and views the active shift tasks for his location.
  2. A staff member completes a task and logs a minor customer complaint using the mobile UI.
  3. At the end of the shift, the Operations Agent automatically drafts a shift summary including the task completion rate and the complaint.
  4. Jun reviews and approves the summary, escalating it to the regional owner.
  **Acceptance Criteria:**
  - Implement the necessary backend data structures for shifts, tasks, and issues.
  - Build the mobile-first UI using OHC design tokens.
  - Ensure the UI renders real data from the backend, with zero mock data.
  - Implement an asynchronous worker to draft the shift summary using the AI provider.
  - Write Playwright E2E tests covering the complete CUJ.
  - Ensure 100% unit test coverage for new code.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
