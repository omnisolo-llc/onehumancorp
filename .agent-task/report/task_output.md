issue_title: "Feature: The Estimator Agent - AI-Automated Quote & Proposal Generator"
issue_description: |
  # Research Report: The Estimator Agent - AI-Automated Quote & Proposal Generator

  ## 1. Problem Statement
  Service-based owners like Carlos (Field Service / Handyman) and Nora (Agency Principal) spend hours manually drafting estimates and proposals after receiving customer inquiries. They often lose leads to competitors because they take too long to reply with a formatted quote. Building and formatting proposals on a mobile device is tedious, error-prone, and frustrating.

  ## 2. Research Report
  - **Market Context**: Legacy field service and CRM platforms (e.g., Jobber, ServiceTitan, HoneyBook) offer quoting tools, but they require the owner to manually input line items, calculate totals, and format the document. They act as passive tools rather than proactive assistants.
  - **The Gap**: OHC currently lacks a native, agent-driven quoting system that automatically bridges the gap between lead intake (Work Triage) and revenue collection (Sales & Revenue).
  - **The OHC Opportunity**: By integrating lead intake forms/DMs, the customer CRM, and the business's service catalog, the **Estimator Agent** can instantly parse a customer request, match it with standard pricing, and draft a complete proposal. The owner simply reviews and approves it on their mobile device.

  ## 3. Design Doc
  ### Data Model (PostgreSQL)
  - `Quote`: Links to `tenant_id`, `customer_id`, `status` (Draft, Sent, Accepted, Rejected, Expired), `total_amount`, and `expires_at`.
  - `QuoteLineItem`: Links to `quote_id`, `catalog_item_id` (optional, for standard services), `description`, `quantity`, `unit_price`.

  ### AI Integration & Architecture
  - **Event Trigger**: Lead intake webhooks (from DMs, emails, or web forms) emit a `LeadReceived` event.
  - **Estimator Agent (Sales/Revenue Dept)**: Listens for `LeadReceived`. Uses RAG to pull the owner's service catalog, pricing guidelines, and past similar quotes.
  - **Draft Generation**: The LLM extracts the requested services, matches them to catalog prices, calculates estimated time/materials, and drafts a structured Quote.
  - **Agent Feed Integration**: The agent creates an `AgentFeedItem` with the `draft_action` containing the quote details, alerting the owner.
  - **Conversion**: Upon owner approval, the quote is sent to the customer with a Stripe Payment Link (for deposits) or converted directly to an Invoice.

  ### Mobile UX Flow (375px)
  1. **Notification**: Owner receives a push notification: "New roof repair inquiry from John. I've drafted an estimate for $450. [Review]".
  2. **Agent Feed Card**: The OHC app feed displays a clear, 375px-optimized card summarizing the quote's line items and total.
  3. **Interaction**: The owner can tap "Approve & Send", "Edit" (which opens a native mobile form to tweak quantities/prices), or "Discard".
  4. **Customer View**: The customer receives an SMS/email with a link to a beautiful, mobile-friendly proposal page where they can accept and pay the deposit.

  ## 4. Implementation Prompt
  **Feature Name**: The Estimator Agent & Quoting Engine

  **Target Persona**: Carlos the Handyman / Nora the Agency Principal

  **Outcome**: When a new service inquiry is captured, the Estimator Agent automatically drafts a structured quote using the business's catalog pricing and presents it in the Agent Feed for 1-tap approval.

  **Critical User Journey (CUJ)**:
  1. Customer submits a request: "I need my gutters cleaned, 2-story house."
  2. The Estimator Agent intercepts the request, looks up the "2-Story Gutter Cleaning" price from the catalog, and drafts Quote #101.
  3. Carlos opens the OHC mobile app and sees the drafted quote in his Agent Feed.
  4. Carlos taps "Approve & Send".
  5. The quote is emailed to the customer with an acceptance button.

  **Next Actions for Engineering**:
  1. Implement the `Quote` and `QuoteLineItem` PostgreSQL schemas with strict row-level security (`tenant_id`).
  2. Develop the `Estimator Agent` worker that listens to lead events, invokes the LLM (Gemini/Minimax) with catalog context, and generates the quote draft.
  3. Integrate the drafted quote into the `AgentFeedService` to surface an action card.
  4. Build the mobile-first (375px) Quote Review UI and the customer-facing Quote Acceptance page.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
