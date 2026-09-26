issue_title: "F12 Reliable Provider Outcome and Approval Reconciliation"
issue_description: |
  **Title**: F12 Reliable Provider Outcome and Approval Reconciliation

  **Problem Statement**:
  Currently, simulation, unknown provider outcomes, and approval paths can falsely appear as completed to the non-technical owner/operator. A small business owner needs absolute certainty that when the system says a client was billed or an artifact delivered, it actually happened. False completion degrades trust and can cause significant business harm.

  **Research Report**:
  Based on `RESEARCH.md` and the remediation ledger, the system must distinguish between simulation/preparation, pending provider response, and verified external execution. Competitors like Shopify and Stripe maintain strict idempotent states and webhook reconciliation. OHC must ensure that generic status changes cannot manufacture payment or delivery states without explicit, provider-verified receipts, and that stale approvals or revocations are respected on affected paths. This supports the "Control authority and cost" and "Run the AI team" responsibilities.

  **Design Doc**:
  - **Architecture diagram (Mermaid.js)**:
    ```mermaid
    sequenceDiagram
      actor Owner
      participant UI
      participant Orchestrator
      participant Agent
      participant Provider
      Owner->>UI: Approves pending task
      UI->>Orchestrator: Submit approval (with auth)
      Orchestrator->>Orchestrator: Verify exact authority & active policy
      Orchestrator->>Agent: Execute authorized external action
      Agent->>Provider: API call with idempotency key
      Provider-->>Agent: Pending / Unknown State
      Agent-->>Orchestrator: Record pending external receipt
      Orchestrator-->>UI: Display waiting/pending state
      Provider-->>Orchestrator: Webhook / Async Reconciliation
      Orchestrator->>Orchestrator: Verify provider receipt
      Orchestrator-->>UI: Update to True Completion (Success/Fail)
    ```
  - **UI wireframes or screen flow description (375px first)**:
    On a 375px mobile layout using macOS-style Translucent Glass materials (`backdrop-filter: blur(30px) saturate(210%)`), a dashboard card displays pending approvals. When tapped, the card expands to show the exact external action. After approval, the card transitions to a "Waiting for Provider" state with a clear visual spinner, rather than immediately showing "Done". Once the provider receipt is verified, it turns into a verified success checkmark with the receipt ID.
  - **Mobile UX flow**:
    1. Owner opens app and sees a "Needs Attention" queue.
    2. Owner taps a draft invoice to approve sending.
    3. The button shows a loading state. The backend synchronously records the intent and initiates the provider action.
    4. The UI reflects a "Sent (Waiting for Delivery Confirmation)" state, ensuring no false completion is presented.
  - **AI agent integration points**:
    Agents cannot self-approve or blindly assume success. When an agent emits an action requiring provider interaction, it must yield to the orchestrator. The orchestrator tracks the idempotency key and waits for external reconciliation before marking the agent's goal as achieved.
  - **Key design decisions and why**:
    - *Explicit Pending States*: Eliminates false completion by adding a first-class "pending reconciliation" state.
    - *Strict Idempotency*: Prevents duplicate charges or deliveries on retries.
    - *Zero-Trust Provider Receipts*: Internal states only transition to "completed" when an external provider receipt is cryptographically verified or firmly acknowledged by an adapter.

  **Implementation Prompt**:
  Implement the Reliable Provider Outcome and Approval Reconciliation (F12) across the core execution and billing paths. Update the domain model to include a "pending reconciliation" state for external actions. Ensure that when a user approves an action, the system uses an idempotency key with the external provider. Do not record the action as successful until the provider's webhook or synchronous response with a valid receipt ID is received. Ensure mobile UI reflects this pending state clearly. Write acceptance tests to verify that simulating an action or encountering a provider timeout does not result in a completed state.

  **Priority**: P0

  **Estimated Scope**: Medium

  **Strategy Admission**:
  Target ID: F12.
  Customer/stage: Nora (agency principal, 39) / Run stage.
  Evidence level: Documented gap in audit ledger.
  Baseline/result metric: Zero false completions / Total provider actions.
  Dependencies/reuse: Reuses existing Orchestrator and UI Next.js components.
  Non-goals: Does not implement generic ERP or new third-party integrations.
  Authority class: Strict owner approval with verified provider receipts.
  Cost plan: Marginal database writes for pending states, no significant compute increase.
  Acceptance checks: Happy path - action approved, pending, receipt received, marked complete. Failure path - action approved, provider timeout, marked pending/failed, no false completion.

  **Superpowers Provenance**:
  Loaded skills: skills/using-superpowers/SKILL.md, skills/writing-plans/SKILL.md, skills/executing-plans/SKILL.md
  Revision: 9b24479e00072bba5c7c8bc7f28ed9baec4b80b0 (approximate latest)
issue_priority: P0
issue_category: research
issue_type: task
issue_label:
  - agent-report
  - ohc:lane:reliability
  - ohc:journey:J1
assignees: []
