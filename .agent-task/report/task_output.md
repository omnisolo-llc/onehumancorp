issue_title: "Agentic Proposal & Deposit Milestone Workflow for Service Operators"
issue_description: |
  ## Problem Statement
  Service operators like Carlos (Handyman) and Nora (Agency Principal) struggle with the friction between initial inquiry, quoting, securing a deposit, and getting paid for milestones. Legacy platforms separate CRM, quoting tools, and payment gateways. Owners often have to manually stitch together an invoice in QuickBooks or Stripe after sending an email proposal, leading to lost leads, delayed payments, and disjointed client communication. They need an integrated, AI-assisted flow that takes an inquiry, auto-drafts a proposal with deposit links, and tracks milestone payments without them ever leaving their mobile device.

  ## Research Report
  - **Competitor Gaps**:
    - *Shopify*: Purely e-commerce; quoting/deposits require heavy apps (e.g., Globo Request a Quote) that don't integrate well with agentic flows.
    - *Stripe*: Powerful invoices/payment links, but no native CRM or proposal generation for non-technical users.
    - *HoneyBook/Dubsado*: Good for service businesses but complex to set up; heavy reliance on templates rather than dynamic AI agents; not mobile-first for quick on-the-go adjustments.
  - **The OHC Opportunity**: By natively linking the `Quote`, `Invoice`, and `Ledger` systems with the **Sales & Revenue Assistant**, OHC can transform a simple text request ("Can you fix my roof next week?") into a fully structured proposal, complete with an embedded 20% deposit link, sent via the customer's preferred channel (SMS/WhatsApp).

  ## Design Doc
  ### Architecture
  ```mermaid
  graph TD
      A[Customer Inquiry via SMS/DM] --> B[Work Triage Agent]
      B --> C[Sales Assistant Agent]
      C --> D[Quote Service]
      D --> E[Stripe Payment Intent / Checkout]
      C --> F[Draft Proposal Card - Mobile]
      F -->|Owner Approves| G[Send to Customer]
      G -->|Customer Pays Deposit| H[Ledger & Booking Service]
      H --> I[Operations Assistant: Schedule Task]
  ```

  ### Mobile UX Flow (375px)
  1. **Intake Notification**: Carlos receives a rich notification: "New Roof Repair Lead - $500 estimated."
  2. **Proposal Review Screen**: Carlos opens the app to a 375px-optimized card showing the AI-drafted proposal. It includes a smart breakdown (Materials, Labor) and a pre-configured 20% deposit.
  3. **One-Tap Action**: A sticky bottom bar presents primary actions: "Approve & Send Link", "Edit Details", or "Decline".
  4. **Deposit Secured State**: Once the customer pays, the view transforms into an Active Project card, and the remaining balance is automatically scheduled as a pending invoice.

  ### AI Agent Integration Points
  - **Sales Assistant**: Intercepts `Lead` events, parses context to generate a structured `Quote` and `QuoteLineItem`s, and proposes a deposit percentage based on historical data or tenant settings.
  - **Operations Assistant**: Once the deposit `PaymentEvent` is confirmed, automatically drafts a `Booking` or task and prompts the owner to finalize scheduling.

  ### Key Design Decisions
  - **Unified State Machine**: A `Quote` transitions to an `Invoice` (deposit) and automatically schedules future milestone invoices.
  - **Stripe Integration**: Leverage Stripe Checkout Sessions with `payment_intent_data.setup_future_usage` for seamless downstream milestone collections without re-asking for card details.
  - **Zero-Friction Mobile Approval**: Eliminate the need for the owner to manually create line items unless they choose to edit the AI's draft.

  ## Implementation Prompt
  - Implement the backend flow connecting `Quote` generation to Stripe Deposit Payment Links.
  - Create the Agent logic for the Sales Assistant to auto-draft the Quote from unstructured inquiry text.
  - Build the 375px mobile-first Proposal Approval Card with "Approve & Send", "Edit", and "Discard" actions.
  - Ensure the state machine correctly handles the transition from "Deposit Paid" to "Project Active" and schedules the remaining balance.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
