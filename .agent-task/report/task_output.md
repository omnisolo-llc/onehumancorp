issue_title: "[Research] Autonomous Agent Department Handoff & State Management"
issue_description: |
  # Architecture Research & Design Doc: Agent Department Handoffs

  ## 1. Problem Statement
  Currently, OHC allows users to configure isolated, built-in agents and single-agent workflows. However, small businesses (e.g. Maya the baker, Nora the agency principal) don't have separate disconnected departments (Sales, Customer Service, Operations). They need unified, cross-functional execution. If Maya receives an Instagram DM for a wedding cake, she needs:
  1. The Customer Service Agent to respond politely.
  2. The Sales Agent to draft a quote.
  3. The Operations Agent to check kitchen capacity and add it to the schedule.

  Today, OHC lacks a unified "Department Handoff" pattern where a centralized orchestrator securely transitions state, intent, and context across specialized AI agents.

  ## 2. Competitive & Market Research
  - **Tencent Workbuddy / WeCom**: Focus on seamless multi-agent orchestration via natural language.
  - **Shopify Sidekick**: Uses specialized sub-agents (Analytics, Content, Code) but maintains a single conversational interface for the user.
  - **Microsoft AutoGen / LangGraph**: Popular open-source frameworks implementing hierarchical or graph-based agent routing with explicit state transitions.
  - **Gap**: OHC needs a robust, non-technical abstraction for this. The "Swarm" needs an invisible routing engine that passes a standard "Task Envelope" between the Customer, Sales, and Ops agents without human intervention unless explicitly configured for "Owner Approval".

  ## 3. Architecture Design: KAIROS Department Routing

  ### 3.1. Overview
  We will introduce the `Department Handoff Protocol` within the KAIROS Orchestration layer. This involves a centralized `TaskEnvelope` passed between distinct Agent Nodes in the Distributed State Machine.

  ### 3.2. Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Source as External (IG DM)
      participant Triage as Triage Department
      participant CS as CS Department
      participant Sales as Sales Department
      participant Owner as Owner UI (Work Feed)

      Source->>Triage: New Message Received
      Triage->>CS: Route TaskEnvelope
      CS->>Sales: Reply Drafted, Request Quote
      Sales->>Owner: Draft Quote Ready
      Owner-->>Sales: Approve
      Sales->>Source: Send Final Response
  ```

  ### 3.3. Data Model
  - `TaskEnvelope`: Represents a unit of work that contains routing history, status, and associated data necessary for cross-agent execution. Needs to strictly support multi-tenancy.

  ### 3.4. Multi-Tenant Isolation
  - All transitions and lock acquisitions in KAIROS must strictly filter on `tenant_id`.
  - Redis Redlock keys must incorporate the `tenant_id`: `ohc:lock:{tenant_id}:envelope:{envelope_id}`.

  ### 3.5. Mobile UX Flow (375px)
  - **The Work Feed**: Instead of viewing separate agent chat windows, the user sees a single unified "Work Feed" card.
  - **Card UI**:
    - Title: "New Custom Cake Inquiry"
    - Body: "Customer Service replied. Sales drafted a $150 quote. Ops confirmed delivery date."
    - Action Button (Translucent Glass): "Approve & Send Quote"
  - **Empty State**: "All clear. No pending tasks." (Truthful, not mocked).

  ## 4. Implementation Prompt
  **Goal:** Implement the backend routing mechanism for the Department Handoff Protocol in the KAIROS Orchestrator (Go) and expose it to the Flutter UI.
  **CUJ:**
  1. A background job creates a new `TaskEnvelope` in the `Triage` department.
  2. The Triage agent processes it and updates the `current_department` to `Sales`.
  3. The UI (Work Feed) reflects this state transition in real-time or via API poll.
  4. The Owner clicks "Approve" which moves the envelope to `Completed`.
  **Acceptance Criteria:**
  - Define the `TaskEnvelope` schema and Postgres migration, keeping in mind the multi-tenancy isolation requirement.
  - Implement KAIROS state transition logic enforcing `tenant_id` isolation.
  - Create a 100% unit-tested Go service for envelope handoffs.
  - Add at least one full E2E Playwright test proving an owner can view and approve an envelope in the UI.
  - Ensure ZERO mocked data in the UI; use the real KAIROS endpoints.

  ## 5. Priority & Scope
  - **Priority:** P1
  - **Scope:** Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
