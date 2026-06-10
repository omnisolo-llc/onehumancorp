issue_title: "Feature: Autonomous B2B Quoting & Proposal Generation Pipeline"
issue_description: |
  # Research Report: Autonomous Quoting & Proposal Generation Pipeline

  ## 1. Problem Statement
  Service-based and B2B small businesses (e.g., Carlos the Field Service Owner, Nora the Agency Principal) waste countless hours manually drafting quotes, estimates, and proposals. They struggle to chase down client approvals and initial deposits. Current e-commerce platforms (like Shopify) lack native service quoting, treating everything as a static product. Dedicated quoting tools (like HoneyBook or Jobber) are effective but require the owner to manually build the quote line-by-line. There is a massive gap for a platform that takes a simple owner prompt ("Draft a quote for Carlos' roof repair, 500 sq ft slate, include 10% discount") and uses an AI assistant to autonomously generate a professional, interactive proposal.

  ## 2. Research Report
  - **Market Context**: Platforms like Jobber and HoneyBook dominate the field service and agency markets by offering strong CRM and quoting workflows. However, these tools are highly manual. The owner must input every detail. Shopify B2B exists but is overwhelmingly tailored to wholesale products, not bespoke service estimates.
  - **The OHC Opportunity**: OHC can differentiate by integrating "The Sales Assistant" directly into the quoting flow. The Assistant can read an unstructured input (e.g., a voice memo or text note), query the tenant's master catalog for pricing rules, draft the proposal, and present it to the owner for a 1-tap approval.
  - **Competitor Gaps**:
    - *Shopify*: Bookings/Quotes are treated as products via complex workarounds; extremely poor native proposal generation.
    - *HoneyBook/Jobber*: Excellent workflows but lack "invisible AI drafting". They are software you have to operate, not an assistant that does the work for you.
    - *Stripe Invoicing*: Great for payment collection but lacks the upstream proposal drafting and negotiation phases.

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Owner Input: Voice/Text] --> B(Work Triage Gateway)
      B --> C{Sales & Revenue Assistant}
      C -->|Lookup Pricing| D[Master Catalog & Rules DB]
      C -->|Lookup Client| E[Unified Customer Graph]
      C -->|Draft Document| F[Proposal Engine]
      F --> G[Action Required Queue]
      G --> H[Mobile App Feed 375px]
      H -->|1-Tap Approve| I[Client Facing Proposal UI]
      I -->|Client Accepts| J[Stripe Deposit Checkout]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  1. **Intake**: The owner taps a "New Quote" FAB and dictates: "Quote for Sarah's kitchen design, standard package plus 5 hours revision."
  2. **Agent Feed**: An "Action Required" card appears in the mobile feed: "Drafted Proposal for Sarah ($1,250)".
  3. **Owner Review**: Tapping the card shows a clean summary of line items, terms, and the total. The owner can tap "Edit" to adjust numbers or "Approve & Send" (massive touch targets).
  4. **Client View**: The client receives an SMS/Email link opening a mobile-optimized webpage to view the proposal, tap "Accept", and immediately pay the required deposit via Stripe.

  ### AI Agent Integration Points
  - **Sales & Revenue Assistant**: The core agent here. It parses the natural language input, matches it to the tenant's configured services (e.g., mapping "standard package" to the `$1000` catalog item), calculates totals, and formats the proposal.
  - **Operations Assistant**: Once the deposit is paid, this agent is triggered to create the corresponding project tasks or service bookings in the owner's schedule.

  ### Key Design Decisions
  - **Proactive Drafting**: Shift the burden of data entry from the owner to the AI.
  - **Integrated Payments**: A proposal is useless without a closing mechanism. The generated document must seamlessly transition into a Stripe Checkout session upon client approval.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Autonomous B2B Quoting & Proposal Generation
  **Target Persona**: Nora the Agency Principal / Carlos the Field Service Owner
  **Outcome**: The owner can generate a complete, accurate, and payable client proposal from a single sentence or voice note, approving it with one tap on their mobile phone.

  **Next Actions for Engineering**:
  1. Implement the core Data Models (`Proposal`, `ProposalLineItem`, `ClientApprovalStatus`) with strict multi-tenant isolation in PostgreSQL.
  2. Develop the AI "Sales Assistant" capability to parse unstructured quoting requests against the tenant's `Service/Product` catalog.
  3. Build the 375px mobile feed UI for the owner to review and approve the drafted proposal.
  4. Build the client-facing proposal acceptance webpage integrated with Stripe for deposit collection.

  **Acceptance Criteria**:
  - The flow must work end-to-end on a 375px viewport without horizontal scrolling.
  - Zero manual line-item entry required by the user in the happy path.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []