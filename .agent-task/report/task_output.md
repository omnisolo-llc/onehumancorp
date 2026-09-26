issue_title: "F12: Enforce truthful provider outcomes and strict approval paths"
issue_description: |
  **Title**: Architect truthful provider outcome tracking and strict approval paths (F12)

  **Problem Statement**:
  Currently, simulation, unknown provider outcomes, and approval paths can appear as completed tasks. For a non-technical owner/operator, a false positive regarding task completion (e.g., an invoice showing as "paid" when the provider hasn't successfully settled, or an action showing as "done" when the underlying API failed) erodes trust. We need exact authority models, stale approval and revocation checks, and strict provider reconciliation checks on all affected paths so that the AI team's progress reflects genuine, confirmed outcomes only.

  **Research Report**:
  - Code audit identified F12: "Simulation, unknown provider outcome and approval paths can look like completion."
  - "Truthful states/receipts; exact authority, stale approval/revocation and reconciliation checks on affected paths" are required.
  - An owner cannot afford to think a customer was billed if the Stripe API timed out or the webhook was missed.
  - Compare to Shopify/Square: Operations sit in a `pending` state with explicit retry capabilities until a confirmed, cryptographically signed receipt is reconciled.
  - We must separate "intent/drafting" from "submission/execution" and from "reconciliation/completion".

  **Design Doc**:

  - Architecture diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Owner as Owner (Mobile/Web)
      participant UI as OHC UI
      participant Agent as AI Operations Agent
      participant Ledger as OHC Ledger
      participant Provider as External Provider (Stripe, etc.)

      Owner->>UI: Approves task / action
      UI->>Ledger: Update state to 'approved_pending_execution'
      UI->>Agent: Dispatch execution command
      Agent->>Provider: Initiate provider action
      alt Provider Success (Synchronous)
          Provider-->>Agent: Success Receipt
          Agent->>Ledger: Write provider receipt, state 'completed'
          UI-->>Owner: Show verified completion with receipt
      else Provider Async / Pending
          Provider-->>Agent: Accepted / Pending Webhook
          Agent->>Ledger: Update state to 'executing_awaiting_receipt'
          UI-->>Owner: Show 'Waiting for provider confirmation'
      else Provider Failure / Simulation
          Provider-->>Agent: Error / Timeout
          Agent->>Ledger: Update state to 'failed_requires_attention'
          UI-->>Owner: Show exact dependency/failure, offer retry
      end
  ```

  - UI wireframes or screen flow description (375px first):
    - **Outcome Feed Screen**: Cards representing tasks. A task in progress shows a pulsing amber "Executing" indicator.
    - **Approval Card**: Large clear "Approve Action" button. Once tapped, transitions to "Sending to provider...".
    - **Exception Card**: Red highlight "Provider Error". Shows exact human-readable reason ("Payment failed") and "Retry" or "Revoke Approval" buttons.
    - **Completion Card**: Green checkmark. MUST include provider receipt ID or transaction reference explicitly shown to prove truthful completion.

  - Mobile UX flow:
    - User opens the app (Mobile-first Translucent Glass materials).
    - Checks the Activity Feed. Sees a pending invoice approval.
    - Taps "Approve and Send".
    - Card immediately switches to "Sending to Stripe..." (Pending state, not Fake Success).
    - Once Stripe webhook fires and is reconciled, the card turns green "Sent (Receipt #12345)".
    - If it fails, card turns amber "Failed to Send" with clear retry button.

  - AI agent integration points:
    - Agents must not mark a task `completed` based on their own successful internal action generation.
    - Agents must query the ledger for explicit provider receipt IDs before summarizing a workflow as "done".
    - Agents monitoring exceptions will generate plain-language explanations of provider errors (e.g., "Stripe says this card is expired").

  - Key design decisions:
    - Isolate 'intent to execute' from 'execution outcome'.
    - Force all provider interactions to produce an exact receipt identifier that the UI must display.
    - Introduce 'stale approval' thresholds (e.g., if approved but not executed within 24 hours, automatically revoke and require re-approval).

  **Implementation Prompt**:
  Implement a strict multi-state tracking mechanism for provider-dependent actions in the core ledger. Add `pending_execution`, `awaiting_receipt`, and `failed` states. Ensure that the Next.js UI component representing tasks strictly requires a `provider_receipt_id` to render the 'completed' state, otherwise falling back to 'pending' or 'failed'. Update the agent orchestration logic to not assume completion after dispatch. Include unit tests that mock a provider timeout and verify the task falls into `failed_requires_attention` rather than `completed`. Do NOT prescribe specific database schemas or API endpoints.

  **Strategy Admission**:
  - Target ID: F12 (Enforce truthful provider outcomes and strict approval paths).
  - Selected customer/stage: Solo digital service professionals (Nora persona), execution and billing stage.
  - Evidence level: Documented code defect (F12) lacking explicit verification paths.
  - Baseline/result metric: 0% false completions in the task feed.
  - Dependencies/reuse: Existing OHC ledger and API provider proxies.
  - Non-goals: Creating new billing providers or changing the provider integration logic, just the state tracking.
  - Authority class: Strict approval path.
  - Cost plan: N/A, internal tracking only.

  **Priority**: P0
  **Estimated Scope**: Medium
issue_priority: "P0"
issue_category: "research"
issue_type: "task"
issue_label: ["agent-report"]
assignees: []