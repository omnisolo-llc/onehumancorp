issue_title: "Implement Zero-Touch Payment Reconciliation & Agentic Invoicing Architecture"
issue_description: |
  ## Product-Use Evidence & Live Stack Discovery
  Attempted to start the OHC service stack with `cd deploy && docker compose -f docker-compose.yml up -d` as mandated. The startup failed due to a host Docker/overlayfs constraint affecting the `valkey` container:
  `Container deploy-valkey-1 Error response from daemon: failed to mount /tmp/containerd-mount2737300451... fstype: overlay ... err: invalid argument`.
  Because the stack cannot launch locally, I am documenting this startup blocker as required and proceeding with the architectural system design based on repository analysis, competitor patterns, and persona needs.

  ---

  ## Title: Implement Zero-Touch Payment Reconciliation & Agentic Invoicing Architecture

  ### Problem Statement
  For non-technical owners like Nora (Agency Principal) and Carlos (Field Service), getting paid requires context switching between operational tools and financial software. Currently, OHC lacks a unified architecture to tie Stripe payments, invoices, and deposits directly to the operational work feed. When Carlos finishes a repair, he must manually open a separate billing view to draft an invoice. The assistant should automatically detect the completed work and present a draft invoice or payment request for approval, eliminating the technical friction of manual reconciliation.

  ### Research Report
  - **Market Context**: Platforms like Shopify seamlessly integrate payments with order fulfillment. However, for service-based businesses (Squarespace, GoDaddy, Wix), invoicing remains a manual, distinct step. Vertical SaaS (like Jobber for field service) links jobs to invoices, but lacks the conversational AI interface.
  - **Competitor Analysis**:
    - **Shopify / Square**: Excellent at physical product payment capture and inventory sync, but rigid for milestone-based service billing.
    - **QuickBooks / FreshBooks**: Complex chart-of-accounts models that overwhelm small operators.
    - **Copilot / Notion AI**: Good at text, but cannot execute financial workflows.
  - **OHC Opportunity**: OHC can differentiate by making the AI Sales & Revenue Assistant the primary interface. The agent observes the state of the task or booking, drafts the payment request (Stripe Payment Link or Checkout Session), and auto-reconciles the payment via webhooks without the user ever opening a "ledger" or "invoices" tab.

  ### Design Doc
  #### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Carlos (Mobile App)
      participant Triage as AI Work Triage
      participant Finance as AI Finance Assistant
      participant Core as OHC API Layer
      participant Stripe as Stripe Gateway

      Owner->>Triage: Marks job "Fix plumbing" as Complete
      Triage->>Finance: Event: Task Completed (ID: 123)
      Finance->>Core: Request Draft Invoice generation for Task 123
      Core-->>Finance: Returns Invoice Draft & Stripe Link
      Finance->>Owner: "Job complete! Send $150 invoice to John?"
      Owner->>Finance: Approves (Tap to Send)
      Finance->>Stripe: Dispatch payment link via SMS/Email
      Stripe-->>Core: Webhook: Payment Intent Succeeded
      Core->>Finance: Reconcile Ledger
      Finance->>Owner: "John paid $150. Transferred to balance."
  ```

  #### Mobile UX Flow (375px First)
  1. **Work Feed Screen**: A Ubiquiti-style translucent card shows the completed job: "Fix plumbing at 123 Main St."
  2. **Assistant Prompt**: A floating action chip appears below the card: "Draft final invoice for $150?"
  3. **One-Tap Approval**: Tapping the chip opens a half-sheet native bottom modal. It displays the line items (Labor: $100, Parts: $50) and a primary CTA: "Send to Customer".
  4. **Confirmation**: The half-sheet dismisses gracefully, and the card updates its badge from "Pending Invoice" to "Sent - Awaiting Payment".

  #### AI Agent Integration Points
  - **AI Finance Assistant (System Prompt update)**: Needs access to `create_invoice_draft`, `generate_payment_link`, and `read_task_context` tools.
  - **AI Work Triage**: Needs to monitor `TaskStatus` transitions and trigger the Finance Assistant when states move to `completed` or `deposit_required`.

  #### Key Design Decisions
  - **Unified Ledger Model**: We will not expose double-entry accounting to the user. The owner sees only "Owed", "Paid", and "Drafts".
  - **Idempotency**: All payment link generation and webhook handling must use strict idempotency keys keyed by `tenant_id` and `task_id` to prevent double-billing on flaky networks.
  - **No-Code Fallback**: Advanced billing settings (tax rates, net-30 terms) are hidden behind a single "Advanced Finance Settings" toggle.

  ### Implementation Prompt
  **To the Implementer Agent:**
  Your goal is to build the end-to-end "Zero-Touch Payment Reconciliation" flow for the owner personas.
  1. **Backend**: Implement the API endpoints and background job queue handlers (using PostgreSQL SKIP LOCKED) to process Stripe webhooks and update the internal ledger.
  2. **AI Tools**: Register new tool calls for the Finance Assistant to draft invoices based on task IDs.
  3. **Frontend (Flutter/PWA)**: Build the 375px-optimized work feed card and the half-sheet approval modal using the OHC Premium Token library (translucent materials, Apple-style hierarchy).
  4. **Acceptance Criteria**: A user must be able to mark a task as complete, immediately receive an AI-generated invoice draft, approve it with one tap, and see the task state update to "Paid" when a mocked webhook is received. Ensure complete row-level security (tenant isolation) and 100% unit test coverage. Do not prescribe specific database column names—design them as needed to support this flow.

  ### Priority
  P1

  ### Estimated Scope
  Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
