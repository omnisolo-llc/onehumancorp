issue_title: "Agentic Omnichannel Quoting & Proposal System"
issue_description: |
  # Agentic Omnichannel Quoting & Proposal System

  ## Title
  Agentic Omnichannel Quoting & Proposal System for Service Operators

  ## Problem Statement
  Service-based owners like Carlos (Handyman) and Nora (Agency Principal) spend hours manually drafting quotes, estimating costs, and chasing client approvals across various channels (email, SMS, WhatsApp). Existing tools (like Jobber or basic Shopify draft orders) are disjointed, require heavy manual data entry, and lack proactive follow-ups. If an owner forgets to follow up, they lose the deal. They need a system that can take a customer request (e.g., from an Instagram DM or an intake form), automatically draft an accurate quote utilizing historical pricing data, and actively manage the follow-up process invisibly.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Jobber / Housecall Pro:** Great for field service operators but very rigid, forms-heavy, and lacking an integrated omnichannel communication layer. Owners have to manually click to send follow-ups.
  - **Shopify B2B / Draft Orders:** Extremely clunky for service estimates. Treating a custom service quote as a "product" is a poor fit for service businesses.
  - **HoneyBook / Dubsado:** Excellent for creatives (like Nora) but often too complex for quick field service estimates. They rely on rigid workflows rather than intelligent, adaptable agentic interactions.
  - **OHC Opportunity:** Implement an "Agentic Quoting Engine." When a lead comes in, the "Sales Agent" drafts the proposal, factoring in parts/labor margins, and pushes it to the owner's Agent Feed for a 1-tap approval. Once sent, the AI automatically tracks views and handles follow-up nudges over the customer's preferred channel (SMS/WhatsApp/Email) until it converts into a deposit/booking.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request via DM/Form] -->|Webhook| B(Omnichannel Inbox)
      B --> C[Intent Resolution LLM]
      C -->|Identifies Request for Quote| D(Sales Agent)
      D -->|Lookups| E[(Service/Inventory DB)]
      D -->|Lookups| F[(Customer Graph DB)]
      D --> G[Draft Quote & Schedule Follow-ups]
      G --> H[Owner Agent Feed Action Card]
      H -->|1-Tap Approve| I(Omnichannel Dispatcher)
      I --> J[Customer Receives Quote Link]
      J -->|Customer Views Quote| K[Telemetry Webhook]
      K --> D
      D -->|Wait 48h if no action| L[Auto Follow-up Draft]
  ```

  ### Mobile UX Flow (375px)
  1. **Agent Feed (Owner):** The owner opens the app and sees a notification card: "Carlos, you have a new quote request for 'Drywall Repair' from John. The Sales Agent drafted an estimate for $450 based on past similar jobs."
  2. **Quote Review Screen:** Tapping the card opens a full-screen, mobile-optimized editor. It features large tap targets to adjust line items (Parts, Labor) and a prominent "Send via SMS" primary action button.
  3. **Customer View:** The customer clicks the link in the SMS to view a visually premium, translucent glass-styled proposal page. It includes a clear summary and a 1-tap "Accept & Pay Deposit" Apple Pay / Stripe Terminal button.
  4. **Post-Acceptance:** The system automatically converts the quote to an Invoice and triggers the Operations Agent to suggest calendar booking slots.

  ### AI Agent Integration Points
  - **Sales Agent:** Analyzes the initial inquiry to construct the line items. Automatically schedules follow-ups based on customer engagement telemetry.
  - **Operations Agent:** Intercedes once the quote is accepted to convert the quote into a scheduled job.
  - **Decision/Finance Agent:** Tracks the win/loss ratio of quotes and alerts the owner if their prices are consistently being rejected compared to local market data.

  ### Key Design Decisions
  - **Centralized Ledger Integration:** Quotes must tie directly into the core Financial Ledger so that accepted quotes instantly generate draft invoices without data duplication.
  - **Zero-Friction Approvals:** The mobile UI must allow an owner to approve a quote in under 5 seconds while at a job site. Editing is available but not mandatory for standard jobs.
  - **Channel Agnostic Delivery:** The system must remember the channel the lead originated from and send the quote link back through that same channel to maintain conversation continuity.

  ## Implementation Prompt
  **User-Facing Outcome:** Implement the core Quoting and Proposal API and Mobile-First UI. Service owners (like Carlos and Nora) should be able to receive a draft quote in their Agent Feed, adjust line items if needed, and send it to the customer. The customer should see a beautiful, mobile-optimized quote acceptance page that processes a deposit via Stripe.

  **Acceptance Criteria:**
  - Create the Quote data model with multi-tenant row-level security.
  - Implement a mobile-first (375px) quote creation and editing screen with premium OHC translucent styling.
  - Implement the customer-facing "Accept Quote" page with a mock or integrated Stripe deposit flow.
  - Ensure the Sales Agent can generate a draft quote object and place it in the owner's Agent Feed queue.
  - Cover the full flow with 100% unit tests and Playwright E2E tests simulating an owner approving a quote and a customer accepting it.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
