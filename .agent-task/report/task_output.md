issue_title: "📏 Architect: Resolve F12 Simulation, Unknown Provider Outcome, and Approval Paths Looking Like Completion"
issue_description: |
  ## Title
  Resolve F12 Simulation, Unknown Provider Outcome, and Approval Paths Looking Like Completion

  ## Problem Statement
  Currently, provider responses that are simulated or unknown, as well as paths awaiting external approvals, can present themselves in the system as "completed." From an owner/operator's perspective (like Maya the baker or Carlos the handyman), this means a booking might look paid when it isn't, or a proposal might look sent when it only generated a simulation. This lack of explicit "waiting", "failed", and "needs-attention" states compromises verified results, business confidence, and safe execution. We need absolute truth in external state transitions: a step is not complete until a confirmed, non-simulated provider receipt or explicit owner/client approval is recorded in the ledger.

  ## Research Report
  - **Context:** The system delegates actions to external providers (e.g., Stripe, Calendar, Mailchimp) and requires approvals for out-of-policy actions.
  - **Current Defect (F12):** "Simulation, unknown provider outcome and approval paths can look like completion. Required remediation: Truthful states/receipts; exact authority, stale approval/revocation and reconciliation checks on affected paths."
  - **Persona Evidence:** Carlos (handyman) relies on a deposit before confirming a booking. If a payment intent is simulated as successful or its state is unknown but treated as complete, he might drive to a job unpaid. Nora (agency) relies on client milestone approvals; if a draft invoice looks like a completed payment, her receivables are incorrect.
  - **Competitive Analysis:** Platforms like Shopify explicitly tag orders as "Payment Pending", "Paid", "Failed". Booking systems like Calendly hold tentative slots until deposit confirmation, clearly marking them. OHC must adopt similarly rigid state machines for all external boundaries.
  - **Baseline:** Current behavior allows state transitions without hard provider receipts.
  - **Goal:** Implement truthful states. No false completions.
  - **Dependencies/Reuse:** Existing `hub.rs` telemetry, `db.rs` for ledgers, and `tool_integrations.rs` for provider adapters.
  - **Non-Goals:** We are not building a generic ERP. We are only resolving the false-completion states for existing workflows.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Owner UI (Mobile)
      participant OHC Coordinator
      participant OHC Ledger (DB)
      participant Provider Adapter (e.g. Stripe)
      participant External Provider

      Owner UI (Mobile)->>OHC Coordinator: Request Action (e.g. Book & Pay)
      OHC Coordinator->>OHC Ledger: Create Intent (State: PENDING_PROVIDER)
      OHC Coordinator->>Provider Adapter: Execute Action
      Provider Adapter->>External Provider: API Call
      External Provider-->>Provider Adapter: API Response (Real or Simulated)

      alt Is Simulated?
          Provider Adapter-->>OHC Coordinator: Outcome: SIMULATED
          OHC Coordinator->>OHC Ledger: Update State: SIMULATED (NOT COMPLETE)
          OHC Coordinator-->>Owner UI (Mobile): Show "Test Mode" Badge
      else Is Live Provider?
          alt Success
              Provider Adapter-->>OHC Coordinator: Outcome: SUCCESS + Receipt ID
              OHC Coordinator->>OHC Ledger: Update State: COMPLETED (Receipt: ID)
              OHC Coordinator-->>Owner UI (Mobile): Show "Completed" Checkmark
          else Failure/Unknown
              Provider Adapter-->>OHC Coordinator: Outcome: FAILED/UNKNOWN
              OHC Coordinator->>OHC Ledger: Update State: FAILED/NEEDS_ATTENTION
              OHC Coordinator-->>Owner UI (Mobile): Show "Action Required" Alert
          end
      end
  ```

  ### UI Wireframes (375px Mobile Viewport)
  - **List View (Jobs/Proposals):**
    - Card component (16px corners, macOS Translucent Glass effect).
    - Clear status badges using Ubiquiti UniFi style:
      - `[ Pending Approval ]` (Yellow text, subtle yellow background)
      - `[ Waiting on Provider ]` (Gray text)
      - `[ Action Required ]` (Red text, pulsing dot for unknown/failed)
      - `[ Completed ]` (Green text, only shown when real receipt exists)
      - `[ Simulated ]` (Purple text, "Test Mode" warning)
  - **Detail View:**
    - If status is `Action Required`, display a prominent "Retry" or "Review Error" button.
    - If status is `Pending Approval`, display "Approve" / "Reject" controls with explicit owner authority verification.

  ### Mobile UX Flow
  1. User opens the app to the "Dashboard / Action Items".
  2. "Needs Attention" items are prioritized at the top (e.g., failed payments, stuck approvals).
  3. User taps a "Pending Approval" item, reviews the exact payload and cost, and approves.
  4. State changes instantly to "Waiting on Provider" with a fluid cubic-bezier easing (entrance ≤ 250ms).
  5. Once the real webhook/receipt arrives, the item transitions to "Completed".

  ### AI Agent Integration Points
  - The orchestrator agent must check the specific state column (`provider_receipt_id`, `approval_status`) before chaining subsequent actions.
  - If an action yields `SIMULATED`, the agent must NOT proceed to steps that require real payment or delivery.

  ## Implementation Prompt
  Implementer Agent: Fix defect F12.
  - Modify the core database ledger models and state machines to strictly separate `PENDING`, `SIMULATED`, `COMPLETED`, `FAILED`, and `NEEDS_ATTENTION`.
  - Enforce that a transition to `COMPLETED` requires a valid, non-simulated `provider_receipt_id` or equivalent explicit cryptographic proof from the external provider.
  - Update the approval tracking logic: stale approvals must require re-approval if the underlying context or policy changes.
  - Expose these truthful states to the Next.js / Tauri frontend, ensuring the 375px mobile UI clearly displays these statuses using the OHC Premium Design Standards (translucent glass, fluid motion).
  - Add unit and E2E tests proving that a simulated provider response or an unknown outcome does NOT result in a `COMPLETED` state in the ledger or UI.

  ## Priority
  P0

  ## Estimated Scope
  Medium

  ## Strategy Admission
  - **Target ID:** OHC-03 (Booking/deposit), OHC-04 (Delivery/review), F12 (Remediation Ledger)
  - **Selected Customer/Stage:** Carlos (Handyman) / Nora (Agency), Days 15-45 (Isolation/budget/deduplication enforcement).
  - **Evidence Level:** Implemented & Test-verified (once complete).
  - **Baseline/Result Metric:** Zero false completions / Total provider interactions.
  - **Dependencies/Reuse:** Existing DB schema and provider adapters.
  - **Non-goals:** Do not rewrite the entire provider adapter system; only enforce state boundaries for completions.
  - **Authority Class:** Owner explicit approval required for non-routine recoveries.
  - **Cost Plan:** Minimal computational cost; saves significant business cost by preventing unpaid fulfillment.
  - **Acceptance Checks:**
    - Happy path: Real provider success -> `COMPLETED`.
    - Failure path: Simulated provider -> `SIMULATED` (Not Complete).
    - Failure path: Network error -> `NEEDS_ATTENTION`.

  ## Superpowers Workflow Provenance
  - **Loaded Skills:** `brainstorming` (skills/brainstorming/SKILL.md), `systematic-debugging` (skills/systematic-debugging/SKILL.md), `using-superpowers` (skills/using-superpowers/SKILL.md)
  - **Repository URL:** https://github.com/obra/superpowers.git
  - **Revision Hash:** 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - **Checks Performed:** Identified defect F12, mapped to user personas and architectural guidelines, produced compliant Automator YAML schema.
  - **Outcomes:** Research report formatted exactly as required, explicitly addressing F12 with no-false-completion state invariants.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
