issue_title: "Implement AI-Native Financial Ledger & Automated Invoicing System"
issue_description: |
  # Mission Queue Protocol: AI-Native Financial Ledger & Automated Invoicing System

  ## Problem Statement
  Small business owners and operators (like Nora the agency principal or Carlos the handyman) currently waste significant time managing disparate financial systems. They manually reconcile bank deposits, draft invoices, send payment reminders, and track unpaid work. Traditional platforms (like QuickBooks or standalone Stripe) require technical/financial context to operate, confusing users and delaying cash flow. OHC needs a real-time, multi-tenant financial ledger integrated with the Finance & Decision Assistant to automate invoicing, payment tracking, and revenue reporting, operating completely invisibly via the agent feed.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Traditional Accounting (QuickBooks, Xero):** Highly complex, uses technical jargon (Chart of Accounts, General Ledger), and requires dedicated setup. Not actionable from a 375px mobile screen.
  - **Payment Gateways (Stripe, Square):** Excellent execution but often siloed from the actual work/tasks (like Nora's client projects or Carlos's service routes).
  - **OHC Opportunity:** By integrating a core double-entry ledger at the tenant level, the **Finance & Decision Assistant** can autonomously observe completed tasks, draft invoices, issue Stripe Payment Links, and automatically reconcile payments when webhooks fire. The owner simply receives an "Approve Invoice" card in their mobile feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Work/Task Completed] -->|Event Mesh| B(Finance Assistant)
      B -->|Query| C[Unified Customer Graph]
      B -->|Draft| D[Invoice Record]
      D --> E[Mobile Action Card: Approve Invoice]
      E -->|User Taps Approve| F[Stripe Integration]
      F -->|Generate Link| G[Dispatch via Omnichannel]
      H[Stripe Webhook: Payment Success] -->|Event Mesh| I[Payment Reconciliation Worker]
      I -->|Update| J[Double-Entry Ledger DB]
      I -->|Notify| K[Mobile Feed: Payment Received]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "Carlos, you finished the plumbing job for Sarah. Ready to invoice $250?"
  - **Interaction:** Tapping the card reveals a beautiful, glassmorphism-styled invoice summary. No complex line-item entry required—the AI inferred it from the booking context.
  - **Action:** A prominent primary button "Send Invoice via SMS" and a secondary "Edit Details". Touch targets are exactly 44x44px or larger.
  - **Post-Action:** Card disappears. A new status pill appears in the top right: "Waiting on $250". Once paid, a push notification and success card appear.

  ### AI Agent Integration Points
  - **Finance & Decision Assistant:** Triggered by `TaskStatus.COMPLETED` events. Uses the tenant's pricing catalog and job context to draft the invoice.
  - **Customer Success Agent:** Responsible for drafting the friendly SMS/Email containing the payment link.
  - **Ledger Invariants:** Multi-tenant PostgreSQL rows protected by RLS (`tenant_id`). All ledger entries must balance to zero (credits = debits).

  ### Key Design Decisions
  - **Zero-Accounting Jargon:** Never expose terms like "Accounts Receivable" or "Reconciliation". Use plain language: "Unpaid", "Paid", "Money In".
  - **Agent-Drafted, Owner-Approved:** The AI drafts the financial request, but the owner must explicitly approve it, ensuring control over cash collection.
  - **Idempotency & Resilience:** All Stripe calls and ledger writes must use idempotency keys (e.g., `ohc:idempotency:{tenant_id}:invoice_{id}`) backed by Redis Redlock to prevent double-billing.

  ## Implementation Prompt
  **User-Facing Outcome:** As an owner, when I mark a job or order as "Done," the OHC app immediately pops up a pre-filled invoice card. I tap "Send," and when the customer pays, my revenue dashboard updates automatically without me doing any data entry.
  **CUJ & Acceptance Criteria:**
  1. A background worker emits a `JobCompleted` event for a specific tenant.
  2. The Finance Assistant agent intercepts the event, drafts an invoice based on the job details, and persists it in a new `invoices` table (RLS enabled).
  3. The system surfaces an "Approve Invoice" card in the mobile agent feed.
  4. Upon user approval, the system generates a Stripe Payment Link and updates the invoice state to `SENT`.
  5. Provide Playwright E2E tests: Seed a completed job, verify the drafted invoice appears in the UI, simulate the "Approve" tap, and verify the UI updates to show the invoice as sent.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
