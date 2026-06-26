issue_title: "Implement AI-Agentic Proposal and Invoicing Workflow (The Closer)"
issue_description: |
  # Research Report: AI-Agentic Proposal and Invoicing Workflow (The Closer)

  ## 1. Problem Statement
  Service-based owners like Nora (Agency Principal) and Carlos (Handyman) lose hours each week drafting custom estimates, building proposals, and chasing deposits. Traditional CRM or invoicing tools (e.g., Quickbooks, HoneyBook, Dubsado) are disjointed from the core communication channels. They require the owner to manually extract details from a DM or email, log into a separate desktop-heavy portal, create a draft, and copy-paste links back to the client. This manual administrative burden creates friction, delays response times, and loses potential revenue.

  ## 2. Research Report
  - **Market Context**: Platforms like HoneyBook and Dubsado are popular among service professionals, but their mobile experiences are often secondary to their complex desktop workflow builders. Invoicing platforms like FreshBooks or Stripe Invoicing are great for payments but lack the AI-driven context extraction to *write* the proposal.
  - **The OHC Opportunity**: OHC can eliminate the manual proposal drafting phase entirely. By leveraging the Work Triage unified inbox and an integrated Finance/Sales Agent ("The Closer"), OHC can automatically detect quotation intents from customer messages, reference standard pricing or past projects, and draft a complete, accurate proposal with a one-tap deposit link.
  - **Competitor Gaps**:
    - *HoneyBook/Dubsado*: Requires heavy manual setup, complex workflow mapping, and is overwhelming on a 375px mobile screen.
    - *Stripe Invoicing*: Excellent for billing, but has zero context about the customer conversation or project scope.
    - *Shopify*: Inherently product-focused; poor support for custom service quotes without expensive third-party apps.

  ## 3. Design Doc
  ### Architecture Diagram (Mermaid)
  ```mermaid
  graph TD
      A[Customer Message/Inquiry] -->|Webhook| B(Omnichannel Gateway)
      B --> C[Work Triage Engine]
      C -->|Detects Quote Intent| D(The Closer - Sales/Finance Agent)
      D -->|Query| E[Tenant Memory & Pricing Catalog]
      D -->|Draft Quote| F[Stripe Invoice/Payment Link API]
      D -->|Generate Card| G[Mobile Agent Feed]
      G -->|User Approves| H[Send Proposal & Deposit Link to Customer]
      G -->|User Edits| I[Native Mobile Keyboard Editor]
      H -->|Customer Pays| J[Auto-Convert to Active Project/Task]
  ```

  ### Mobile UX Flow (375px First)
  1. **Work Triage Feed**: Carlos receives an alert in his OHC app: "Sarah requested a quote for kitchen cabinet repair."
  2. **Agent Proposal Card**: Below the message, a translucent glassmorphism card from "The Closer" agent appears. It states: "Drafted Proposal: $450 total ($150 deposit). Based on your standard hourly rate and material estimates."
  3. **Action Buttons**: Large (44x44px minimum) touch targets for **"Approve & Send"**, **"Review/Edit"**, and **"Dismiss"**.
  4. **Edit Mode**: If tapped, a clean native mobile form appears allowing Carlos to quickly adjust the price or add a line item before sending.
  5. **Client Experience**: The client receives a polished, mobile-responsive web link detailing the scope with a Stripe integration for an immediate Apple Pay/Google Pay deposit.

  ### AI Agent Integration Points
  - **Intent Recognition**: The Work Triage system uses LLMs to classify incoming messages as "Quote Request".
  - **Contextual Drafting**: The Closer agent retrieves context from the tenant's past similar jobs, standard catalog pricing, and the specific customer request to formulate line items and scope descriptions.
  - **Action Execution**: Upon approval, the agent automatically interfaces with Stripe APIs to generate the Payment Link or Invoice and dispatches the message via the original communication channel.

  ## 4. Implementation Prompt
  **User-Facing Outcome**: When a customer messages a service owner requesting an estimate, the owner should see a pre-drafted proposal and deposit request in their mobile feed, ready for one-tap approval.

  **Critical User Journey (CUJ)**:
  1. Owner receives a simulated customer inquiry via WhatsApp/Email requesting a custom service.
  2. The backend event mesh triggers the Sales/Finance agent.
  3. The owner opens the OHC mobile app (375px view) and sees an "Action Required" card containing the drafted quote.
  4. The owner taps "Approve & Send".
  5. The system generates a Stripe payment link and dispatches the response back to the customer.

  **Acceptance Criteria**:
  - Create the data schema for `Proposal` and `LineItem` tied to a `Tenant` and `Customer`.
  - Implement the background worker that listens to the Work Triage event mesh for quotation intents.
  - Integrate the LLM prompt chain that drafts the proposal based on message context.
  - Build the 375px mobile UI card component using OHC Premium Token glassmorphism standards.
  - Implement Stripe API integration for deposit collection.

  ## 5. Priority & Scope
  - **Priority**: P1
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
