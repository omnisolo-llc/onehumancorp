issue_title: "Validate and complete inquiry-to-proposal-to-payment execution (OHC-04/05)"
issue_description: |
  ## Mission Queue Protocol: Architectural Brief

  ### Title
  Validate and complete inquiry-to-proposal-to-payment execution for service professionals (OHC-04/05)

  ### Problem Statement
  OneHumanCorp's core promise to service professionals (e.g., Nora, the agency principal, or Carlos, the handyman) is moving unorganized customer intent into clear, verified, revenue-generating actions. Currently, while there are existing mockups and disconnected backend stubs for handling proposals, invoices, calendars, and a Stripe client, there is no end-to-end verified workflow that seamlessly converts an initial customer inquiry into an accurate proposal, leading directly to a usable payment request and subsequent booking. This missing link breaks the critical "Acquisition → Onboarding → Revenue" customer journey loop. We must replace or disable any placeholder behavior with verified, robust integration evidence.

  This is a critical gap. As identified in `docs/research/business_capability_and_usage_economics_audit.md` (Table 8):
  > **Evidence question:** Can an inquiry become an accurate proposal and a usable payment request?
  > **Reuse / gap to investigate:** Existing proposals, invoices, calendars and Stripe client; replace or disable placeholder behavior with evidence.
  > **Decision unlocked:** Whether customer-to-payment execution is a viable first workflow and for which owner type.

  ### Research Report
  - **Market Context:** Solo professionals rely heavily on fast proposal generation and frictionless deposit collection to secure work. Delays in converting a DM or email inquiry to a paid booking directly result in lost revenue. Competing tools (e.g., HoneyBook, Dubsado) handle this well, but often require heavy initial setup.
  - **OHC Advantage:** OHC can utilize AI to draft the proposal based on a brief inquiry, and leverage existing Stripe integration for instant payment.
  - **Current State Audit:** The existing codebase contains fragmented parts (Stripe integration, some proposal generation logic), but they are disjointed. The business logic fails to enforce robust state transitions from "inquiry received" -> "draft proposal generated" -> "proposal approved by owner" -> "payment link sent" -> "deposit received / job booked."
  - **Competitor Baseline:** Platforms like Shopify or GoDaddy are better suited for immediate product checkout; OHC's target needs a "quote-to-cash" flow suitable for asynchronous service delivery.

  ### Design Doc
  - **Mobile UX Flow (375px first):**
    1.  **Inbox/Feed:** Owner receives a notification of a new inquiry. They tap to view a summarized card.
    2.  **Draft Proposal:** AI generates a draft proposal (scope, price, terms) based on the inquiry and the owner's configured business logic.
    3.  **Review & Approve (Grandmother Test):** Owner views a clean, macOS-style Translucent Glass card containing the draft proposal. They can tap "Approve & Send" or "Edit." The terms and Stripe payment details (deposit amount) are clearly visible. No technical jargon.
    4.  **Customer Experience:** Customer receives a simple, mobile-optimized link to review the proposal and pay the deposit via Stripe checkout.
    5.  **Confirmation:** Upon payment, the OHC dashboard updates the job state to "Booked" and notifies the owner.

  - **Architecture / Data Model (Mermaid.js conceptual):**
    ```mermaid
    sequenceDiagram
        participant Customer
        participant OHC_Agent
        participant OHC_Backend
        participant Stripe

        Customer->>OHC_Agent: Submits Inquiry
        OHC_Agent->>OHC_Backend: Create Inquiry Record
        OHC_Backend->>OHC_Backend: AI Drafts Proposal & Pricing
        OHC_Backend->>OHC_Agent: Request Owner Approval
        Note over OHC_Agent: Owner Reviews & Taps "Approve"
        OHC_Agent->>OHC_Backend: Confirm Proposal
        OHC_Backend->>Stripe: Generate Payment Link (Deposit)
        OHC_Backend->>Customer: Send Proposal + Payment Link
        Customer->>Stripe: Completes Payment
        Stripe-->>OHC_Backend: Webhook (Payment Succeeded)
        OHC_Backend->>OHC_Backend: Update Job State to "Booked"
        OHC_Backend->>OHC_Agent: Notify Owner (Success)
    ```
    - **Invariants:**
      - A proposal cannot be sent without explicit owner approval (Zero Trust on financial commitments).
      - Payment state must be reconciled via Stripe webhooks (no assumed success).
      - Tenant data isolation must be strictly enforced throughout the chain.

  - **AI Agent Integration Points:**
    - **Intake parsing:** Extracting scope, timeline, and customer details from raw text.
    - **Proposal generation:** Drafting professional text combining the intake data with the owner's service template.
    - **Notification:** Informing the owner and waiting for the "Approve" signal.

  ### Implementation Prompt
  "Implement the end-to-end 'Inquiry to Paid Proposal' workflow. Start from the existing Stripe client and proposal modules.
  1. Create the backend state machine that transitions an Inquiry to a Draft Proposal, to an Approved Proposal, to a Payment Pending state, to a Booked state.
  2. Implement the mobile-first UI for the owner to review and approve the AI-drafted proposal. Ensure the UI adheres to the Translucent Glass design tokens and the Grandmother Test (clear action buttons, no tech jargon).
  3. Wire up the Stripe webhook handler to reliably transition the state from Payment Pending to Booked upon successful payment.
  4. Write comprehensive integration tests verifying the full flow, explicitly mocking the Stripe API responses to simulate successful and failed payments. Do not build new databases; use existing tables and expand them only if strictly necessary."

  ### Priority
  P0

  ### Estimated Scope
  Medium

  ### Strategy Admission
  - **Target ID:** OHC-04/05
  - **Selected Customer/Stage:** Nora (agency principal) / Launch & Run stage.
  - **Evidence Level:** Needs test-verified and provider-sandbox-verified implementation.
  - **Baseline/Result Metric:** 100% of generated proposals must require explicit owner approval; 100% of successful test-mode Stripe payments must reliably update the job state within 5 seconds.
  - **Dependencies/Reuse:** Existing Stripe client, proposal templates, notification feed.
  - **Non-goals:** Subscription billing (out of scope for this specific transaction flow), complex milestone-based invoicing (keep it to deposit/single payment for now).
  - **Authority Class:** Requires explicit owner approval before sending proposal/payment link.
  - **Cost Plan:** Minimal API calls for proposal drafting. Stripe processing fees apply (handled external to OHC computing cost).
  - **Acceptance Checks:** Happy path (inquiry -> approve -> pay -> booked); Failure path (payment failed -> notify owner, job remains pending).

  ### Context & Provenance
  - **Workflow Used:** superpowers/skills/brainstorming
  - **Revision:** 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - **Source Material:** `docs/research/business_capability_and_usage_economics_audit.md`, `RESEARCH.md`

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
