issue_title: "[Research] Autonomous Invoicing & Payments Agent"
issue_description: |
  # Research Report: Autonomous Invoicing & Payment Collection Agent

  ## 1. Problem Statement
  Small business owners (e.g., Nora the Agency Principal, Carlos the Field Service Owner) spend significant time manually drafting, sending, and tracking invoices. Follow-ups for late payments are awkward, time-consuming, and often forgotten, leading to cash flow issues. Traditional platforms require manual creation of invoices and manual follow-up reminders. They lack an intelligent, proactive system that drafts invoices based on completed work and autonomously chases down late payments.

  ## 2. Research Report
  - **Market Context**: Existing tools like QuickBooks, FreshBooks, or Stripe Billing require the user to log in, manually input line items, and set up rigid, rule-based reminders. They are tools, not assistants.
  - **The OHC Opportunity**: Integrate an AI agent (The Finance Assistant) that observes when a project or job is marked "complete" (e.g., Nora finishes a design milestone, Carlos completes a repair), automatically drafts the invoice with correct line items and taxes, and presents it to the owner for 1-tap approval. Once sent, the agent autonomously monitors payment status and drafts personalized, polite follow-ups if the payment is late.
  - **Competitor Gaps**:
    - *Stripe/QuickBooks*: Powerful but require manual data entry and rigid configuration.
    - *Shopify*: Primarily e-commerce; poor support for custom B2B or service-based invoicing.
    - *OHC*: Can seamlessly bridge the gap between "work completed" and "payment collected" with AI.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Work Management Module] -->|Job/Project Completed| B(Event Bus)
      B --> C[Finance Assistant Agent]
      C -->|Query| D[Customer/Project Database]
      C -->|Draft Invoice| E[Action Required Queue]
      E --> F[Mobile App Feed 375px]
      F -->|1-Tap Approve| G[Invoice Service / Stripe]
      G -->|Send via Email/SMS| H[Customer]
      G -->|Webhook: Payment Pending| C
      C -->|Draft Reminder after X days| E
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile)**: Top card shows "Action Required: Draft Invoice for Sarah's Kitchen Repair".
  - **Interaction**: Tapping the card opens the draft invoice. The AI has already populated line items (e.g., "Labor - 3 hours", "Parts - $50") based on the completed job details.
  - **Action**: Primary button "Approve & Send", Secondary button "Edit".
  - **Follow-up Flow**: If an invoice is 3 days late, a new card appears: "Sarah's payment is late. Send polite reminder?" with a pre-drafted message.

  ### AI Agent Integration Points
  - **Finance Assistant Agent**: Triggered by job completion events. Uses LLM to extract line items from job notes/quotes and drafts the invoice. Also monitors payment status and drafts context-aware reminders.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Autonomous Invoicing & Payments Agent
  **Target Persona**: Nora the Agency Principal / Carlos the Field Service Owner
  **Outcome**: Owners no longer manually create invoices. The AI drafts them upon job completion, and owners simply tap "Approve" on their phone. The AI also drafts and suggests polite follow-ups for late payments.

  **Next Actions**:
  1. Define the data models for `Invoice`, `InvoiceLineItem`, and `PaymentStatus` with strict tenant isolation.
  2. Implement the Finance Assistant Agent logic that listens for "Job Completed" events and drafts the invoice.
  3. Build the 375px mobile UI cards for "Review Draft Invoice" and "Review Late Payment Reminder".
  4. Integrate with Stripe (or equivalent) for actual payment collection and webhook handling for status updates.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
