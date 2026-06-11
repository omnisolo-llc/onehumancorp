issue_title: "[Research] Distributed Transaction Coordination for AI Agents"
issue_description: |
  # [Research] Distributed Transaction Coordination for AI Agents

  ## Problem Statement
  As OneHumanCorp scales, we are introducing more complex agentic workflows where multiple specialized AI agents (e.g., Sales Assistant, Operations Assistant, Finance Assistant) need to collaborate to fulfill a user request. Currently, we lack a robust distributed transaction coordination mechanism to ensure consistency when these agents perform state-mutating actions across different domains and microservices. Without this, a multi-agent workflow (like booking a service, taking a deposit, and updating inventory) could fail halfway, leaving the system in an inconsistent state (e.g., payment taken but service not booked).

  ## Research Report
  - **Current State:** OHC uses PostgreSQL for the core database and Redis for simple locking. However, there's no native saga pattern or distributed transaction manager implemented for coordinating multi-step processes spanning different agent domains.
  - **Competitor Analysis:**
    - Shopify: Uses extensive background job queues with strict idempotency and saga-like compensating transactions for complex operations like order fulfillment.
    - Stripe: Heavily relies on idempotency keys and state machines to manage complex payment flows, ensuring transactions are eventually consistent or explicitly rolled back.
    - Standard Microservices Patterns: The Saga pattern (Choreography or Orchestration) is the industry standard. Orchestration (using a central coordinator) is often easier to monitor and debug than Choreography.
  - **Proposed Approach for OHC:** Implement a Saga Orchestration engine tailored for AI agent workflows. An "Orchestrator Agent" or central job engine will manage the overall transaction, invoking specialized agents for individual steps. If a step fails, the orchestrator triggers compensating actions in previously successful agents to roll back the state.

  ## Design Doc
  - **Architecture Diagram (Mermaid.js):**
    ```mermaid
    sequenceDiagram
      participant User
      participant Orchestrator
      participant AgentA
      participant AgentB
      User->>Orchestrator: Start Saga
      Orchestrator->>AgentA: Execute Step 1
      AgentA-->>Orchestrator: Success
      Orchestrator->>AgentB: Execute Step 2
      AgentB-->>Orchestrator: Failure
      Orchestrator->>AgentA: Compensate Step 1
      AgentA-->>Orchestrator: Compensated
      Orchestrator-->>User: Saga Failed (Rolled Back)
    ```
  - **UI Wireframes or screen flow description (375px first):**
    - Screen 1: "Confirm & Book" (shows summary of actions to be taken).
    - Screen 2: Loading State (translucent glass modal overlay with "Processing booking, please wait...").
    - Screen 3 (Success): Confirmation page with "Done" button.
    - Screen 3 (Failure): Error state with "Booking failed, any charges have been refunded. Try again."
  - **Mobile UX Flow:** The user initiates the complex action (e.g., tapping "Book & Pay" on a 375px screen). The UI shows a translucent glass loading state indicating "Processing...". The orchestrator kicks off the saga. If successful, the UI updates to the confirmation screen. If the saga fails and rolls back, the UI shows a clear error message (e.g., "Could not complete booking. Any pending charges have been refunded.") and allows the user to retry.
  - **AI Agent Integration:** Agents receive commands with a `saga_instance_id`. They execute their task, report success/failure back to the coordinator, and are prepared to receive a `compensate` command for that same `saga_instance_id` if needed later.
  - **Key Design Decisions:** Use Orchestration over Choreography to centralize tracking for the "Decision & Reporting" capability.

  ## Implementation Prompt
  **User Facing Outcome:** When an owner or a customer initiates a complex action (like booking a multi-step service that involves calendar scheduling, payment processing, and inventory reservation), the system guarantees that either all steps succeed or any partial changes are completely rolled back. They never see a "half-completed" state.

  **Acceptance Criteria:**
  - Create the core `SagaCoordinator` service in Go.
  - Define the PostgreSQL schema for tracking saga execution state.
  - Implement a sample saga (e.g., `ExampleMultiStepSaga`) demonstrating forward execution and successful compensation on simulated failure.
  - Write unit tests ensuring saga state transitions correctly and compensation logic is triggered on step failure.
  - Ensure the solution relies on the existing database and worker infrastructure.
  - No specific library prescription; focus on robust state tracking and error handling.

  ## Estimated Scope
  Large

  ## Priority
  P1

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
