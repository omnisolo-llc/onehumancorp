issue_title: "[Architecture] KAIROS Multi-Agent Departmental Orchestration & State Handoff Mesh"
issue_description: |
  ### Problem Statement
  In OneHumanCorp (OHC), the AI Assistant capabilities (Work Triage, Customer Assistant, Operations, Sales, Finance, Knowledge) must work as a coordinated team. Currently, disjointed LLM calls can lead to race conditions and context loss. For example, when Maya receives overnight custom cake inquiries, multiple agents need to process them: Triage groups them, Customer drafts a reply, Operations checks calendar availability, and Sales generates a Stripe deposit link. Without a unified orchestration mesh, agents might overwrite each other's drafts or send conflicting recommendations to the owner feed. We need a robust architecture for **KAIROS Multi-Agent Departmental Orchestration** that guarantees multi-tenant isolation, cross-agent locking, and graceful state handoffs for the owner's unified action feed.

  ### Research Report
  We evaluated various multi-agent orchestration frameworks (LangGraph, AutoGen, CrewAI) and distributed state machines (Temporal, AWS Step Functions).
  - **Competitor Insights:** Shopify Sidekick is a single bot context; it struggles with parallel departmental reasoning. Square has no cross-domain AI coordination. Notion AI is document-bound.
  - **OHC's Differentiation:** OHC requires agents to run asynchronously in the background via PostgreSQL `SKIP LOCKED` job queues, coordinating via Redis distributed locks (Redlock) to update the unified owner feed (375px mobile UI) without blocking the UI thread.
  - **Key Findings:** We must implement a "Blackboard" or "State Handoff" pattern where a centralized KAIROS Orchestrator routes the event (e.g., `Instagram_DM_Received`) through specific Agent Departments sequentially or in parallel, collecting their outputs before materializing a single "Triage Action Item" for the owner.

  ### Design Doc

  **Mobile UX Flow (375px First):**
  1. Owner opens the OHC mobile app. The "Command Center" feed shows a single translucent Glassmorphism card: "3 New Cake Inquiries (Pending your review)".
  2. Tapping the card opens a unified view where the owner sees:
     - The original customer messages.
     - **Operations Note:** "You have capacity on Friday."
     - **Sales Note:** "Suggested deposit: $50 per cake."
     - **Customer Assistant Draft:** A pre-written Instagram DM reply combining all these facts.
  3. The owner taps "Approve & Send". The KAIROS mesh executes the Stripe link generation and dispatches the IG DM asynchronously.

  **Architecture Diagram:**
  ```mermaid
  sequenceDiagram
      participant Webhook as IG Webhook / API
      participant PGQueue as PostgreSQL (SKIP LOCKED)
      participant KAIROS as KAIROS Orchestrator
      participant RedisLock as Redis (Redlock)
      participant DeptOps as Operations Agent
      participant DeptSales as Sales Agent
      participant DeptCS as Customer Assistant
      participant Feed as Owner Triage Feed (Flutter/PWA)

      Webhook->>PGQueue: Enqueue `WorkIntakeEvent`
      KAIROS->>PGQueue: Dequeue Event
      KAIROS->>RedisLock: Acquire `ohc:lock:tenant123:work_intake:msg456`
      KAIROS->>DeptOps: Request Availability Check
      DeptOps-->>KAIROS: Returns Capacity = True
      KAIROS->>DeptSales: Request Deposit Quote
      DeptSales-->>KAIROS: Returns Quote = $50
      KAIROS->>DeptCS: Draft Final Reply
      DeptCS-->>KAIROS: Returns Draft Message
      KAIROS->>Feed: Materialize Unified Triage Card
      KAIROS->>RedisLock: Release Lock
      Feed-->>Owner: Displays combined action card on 375px screen
  ```

  **AI Agent Integration Points:**
  - **Trigger:** Webhooks, Emails, API inserts.
  - **State Machine:** KAIROS stores the intermediate state of the workflow in a `kairos_agent_handoffs` PostgreSQL table (isolated via RLS `tenant_id`).
  - **LLM Provider Swap:** The Orchestrator uses the `OHC_LLM_PROVIDER` environment variable (Gemini Pro primary, fallback GPT-4o) and passes tenant-scoped memory implicitly.
  - **Locking:** Uses `ohc:lock:{tenant_id}:{resource_type}:{resource_id}` via Redis to prevent concurrent agent processing on the same lead.

  ### Implementation Prompt
  **To the Implementer:**
  Design and implement the `KAIROS Orchestrator` module and the `Work Triage` action feed integration.
  1. **Critical User Journey (CUJ):** Maya receives an inquiry. The system must process it through Operations (availability) and Sales (quote) before presenting a unified draft reply in the mobile Command Center.
  2. **Outcome:** A new database abstraction for multi-agent workflows and the corresponding backend gRPC/REST endpoints. The frontend must display this grouped action card flawlessly on a 375px screen with 44x44px touch targets.
  3. **Acceptance Criteria:**
     - 100% unit test coverage for the Orchestrator service.
     - Playwright E2E test verifying a multi-department background job resolving into a single UI card without race conditions.
     - Redis Redlock is utilized correctly to prevent concurrent duplicate event processing.
     - No hardcoded paths or UI fake data; all data must flow through the real K8s/Docker compose stack.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
