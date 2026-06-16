issue_title: "Agentic Smart Estimates & Automated Deposit Follow-Up Engine"
issue_description: |
  ## Title
  Agentic Smart Estimates & Automated Deposit Follow-Up Engine

  ## Problem Statement
  Service-based small business owners (like Carlos the Handyman or Nora the Agency Principal) lose substantial revenue due to friction in the quoting and deposit collection process. Currently, when a lead requests a quote, the owner must manually calculate the estimate, format a document, send it via email or SMS, and remember to follow up. Often, quotes are sent but deposits are not collected promptly, leading to lost bookings and stalled cash flow. Traditional CRMs or invoicing tools require manual tracking of "sent" vs "accepted" vs "paid" states.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Joist / Invoice2go:** Excellent for manual creation of professional estimates on mobile, but lack AI autonomy to draft the quote based on natural language or past jobs. Follow-ups are rigid, scheduled emails rather than context-aware nudges.
  - **HubSpot / Salesforce:** Overly complex for solopreneurs. They require setting up intricate sales pipelines and automation workflows that non-technical users struggle with.
  - **Stripe Invoicing:** Good for the final payment step but doesn't handle the upstream workflow of estimating a dynamic job scope via customer conversation.
  - **OHC Opportunity:** Implement "The Closer" (Sales Agent) workflow. When an inquiry comes in, the agent parses the scope, checks past similar jobs in the tenant's history, drafts a line-item estimate, and proposes it to the owner. Once approved and sent, the agent autonomously monitors the state and drafts gentle, contextual follow-ups via the customer's preferred channel if the deposit isn't paid within a specified timeframe.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Inquiry] -->|Intake Form / DM| B(Work Triage / Event Mesh)
      B --> C[The Closer Agent]
      C -->|Query| D[Tenant Past Quotes & Pricing Rules DB]
      C -->|Draft| E[Smart Estimate Proposal]
      E --> F[Mobile Agent Feed 375px]
      F -->|Owner Approves| G[Stripe Payment Link / Quote Dispatch]
      G --> H{Deposit State Machine}
      H -->|State: Pending (48hrs)| I[The Closer Agent]
      I -->|Draft Contextual Follow-up| F
      H -->|State: Paid| J[Operations Agent - Schedule Job]
  ```

  ### Mobile UX Flow (375px First)
  - **Agent Feed:** A high-priority card appears: "Drafted Estimate for [Customer Name]: Plumbing Repair ($450)."
  - **Review Screen:** Tapping the card opens a clean, mobile-optimized view of the line items. The owner can tap any line to edit the price or description using a native mobile keyboard.
  - **Action:** A prominent "Send Quote & Request 50% Deposit" button.
  - **Follow-Up Feed Card:** Two days later, a new card appears: "[Customer Name] hasn't paid the deposit for the plumbing job. Want me to send a quick SMS follow-up?" with a "Yes, send it" button.

  ### AI Agent Integration Points
  - **The Closer Agent:** Triggered by new leads in the intake queue. Uses RAG against previous successful quotes to suggest accurate pricing for similar scopes of work.
  - **State Machine Synchronization:** The agent subscribes to Stripe webhook events (e.g., `checkout.session.completed` for deposits) to update the `Quote` status and cancel pending follow-up tasks in the AI Job Queue.

  ### Key Design Decisions
  - **Contextual Pricing Memory:** The agent learns from the owner's past edits. If the owner consistently raises the suggested price for "emergency weekend repairs," the agent adjusts future drafts automatically.
  - **Frictionless Deposit:** The estimate and the payment link are unified. The customer accepts the quote by paying the deposit, reducing the number of steps to secure the booking.

  ## Implementation Prompt
  **User-Facing Outcome:** Carlos the Handyman gets a text: "I need my sink fixed." He opens the OHC app, and the Closer Agent has already drafted a $150 quote with a $50 deposit link based on his standard sink repair rate. He taps "Approve." Two days later, if the customer hasn't paid, the app asks Carlos, "Send a reminder?" He taps "Yes."

  **CUJ & Acceptance Criteria:**
  1. An incoming work request event containing a description (e.g., "fix leaky sink") is ingested.
  2. The Closer Agent queries the database for the tenant's pricing on similar past items and drafts a `Quote` record with line items and a required deposit amount.
  3. The draft appears in the mobile UI feed.
  4. The owner approves the quote via the UI, triggering the generation of a Stripe Payment Link associated with the quote.
  5. A scheduled background job (simulated or real) checks for unpaid quotes after 48 hours and creates a drafted follow-up message in the Agent Feed.
  6. E2E Test: A user logs in, sees the drafted quote, approves it, and the system transitions the quote state from `Draft` to `Sent`.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []