issue_title: "Implement AI-Powered Field Service Quote Generator for Carlos Persona"
issue_description: |
  # Research Report: AI-Powered Field Service Quote Generator

  ## 1. Problem Statement
  Field service owners like Carlos (Handyman) spend hours each evening drafting quotes, estimating costs, and chasing deposits for incoming service requests. Traditional platforms (like Jobber or Housecall Pro) are powerful but require manual data entry on complex forms, often poorly optimized for 375px mobile screens. Carlos needs a system that intakes natural language requests (e.g., via SMS or WhatsApp), instantly drafts a structured estimate or quote, and secures a deposit without manual admin work.

  ## 2. Research Report
  - **Market Context**: Legacy field service management (FSM) tools (Jobber, Housecall Pro, ServiceTitan) are feature-rich but have steep learning curves and require manual quote construction. Link-in-bio tools lack the complex backend for quoting and scheduling.
  - **The OHC Opportunity**: OHC can capture the micro-SME field service market by replacing the manual quote builder with an AI Operations Agent. The owner simply approves a drafted quote from a unified mobile feed.
  - **Competitor Gaps**:
    - *Jobber*: Desktop-heavy setup, manual line-item entry.
    - *Thumbtack*: Takes a massive cut, generic lead interface.
    - *Shopify*: Fundamentally broken for service businesses requiring custom quotes before purchase.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request via DM/Form] --> B[Work Triage Agent]
      B --> C{Operations Agent}
      C -->|Drafts| D[Estimate / Quote]
      C -->|Drafts| E[QuoteLineItem]
      D --> F[Mobile Owner Feed]
      F -->|One-Tap Approve| G[Stripe Payment Link for Deposit]
      G --> H[Sent to Customer via Customer Success Agent]
  ```

  ### Mobile UX Flow (375px)
  1. **Owner Feed View**: Carlos opens the app. The top card shows an AI Proposal: "New lead (broken pipe) wants a quote. I've drafted an Estimate for $150-$250 based on standard rates. [Review & Send]"
  2. **Quote Review**: Tapping the card reveals a clean, translucent glass UI showing drafted line items (`QuoteLineItem`) and required deposit (`required_deposit_cents`).
  3. **Approval**: A large, 44px+ touch target button at the bottom says "Approve & Request Deposit".
  4. **Customer View**: The customer receives a mobile-friendly link to view the quote and pay the deposit via Stripe.

  ### AI Integration
  - **Work Triage Agent**: Parses incoming messages into `ServiceLead` records.
  - **Operations Agent**: Translates the `ServiceLead` description into a structured `Estimate` or `Quote` with `QuoteLineItem`s based on the owner's pricing history, and sets a `required_deposit_cents`.

  ## 4. Implementation Prompt
  **Feature Name**: AI-Powered Quote & Estimate Generator
  **Target Persona**: Carlos the Handyman
  **Outcome**: Carlos receives AI-drafted quotes in his mobile feed based on customer inquiries. He can approve them with one tap, triggering an automated message to the customer with a deposit payment link.

  **Next Actions**:
  1. Implement the integration between the AI Operations Agent and the `Estimate`, `Quote`, and `QuoteLineItem` data models. The agent should parse natural language leads and generate these records.
  2. Build the Mobile-First (375px) Quote Review Card in the Unified Agent Feed, allowing the owner to modify or approve the drafted quote.
  3. Wire the "Approve" action to the Stripe Payment Link generation for the required deposit.
  4. Add automated E2E Playwright tests covering the flow from lead intake to owner approval and customer link generation.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
