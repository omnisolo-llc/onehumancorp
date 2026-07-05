issue_title: "The Negotiator Agent: AI-Driven Custom Quotes & Dynamic Pricing"
issue_description: |
  ## Title
  The Negotiator Agent: AI-Driven Custom Quotes & Dynamic Pricing for Services

  ## Problem Statement
  Service-based small business owners (e.g., Nora the Agency Principal, Carlos the Handyman) spend a disproportionate amount of time turning incoming leads into actionable quotes and proposals. Existing platforms (Shopify, Wix) treat services as static products with fixed prices, which completely fails for custom requests (e.g., "I need a 3-page custom website designed" or "My sink is leaking and the drywall is damaged"). Business owners are forced to manually parse the request, calculate costs, draft a proposal, and negotiate via email. This creates a massive bottleneck, slows down response times, and results in lost revenue.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify/Wix:** Fundamentally designed for fixed-price SKUs. They require third-party apps for "request a quote," which usually just send a static email form to the owner. There is zero AI assistance in pricing or drafting.
  - **HoneyBook/Dubsado:** Excellent for managing proposals *after* the owner manually creates them, but they don't proactively negotiate or price based on initial intake.
  - **OHC Opportunity:** By introducing "The Negotiator" Agent, OHC can autonomously parse custom service requests, reference the owner's past pricing history and internal rate card, and draft a tailored proposal instantly.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Intake Form / DM] --> B(Demand Intake Gateway)
      B --> C[The Negotiator Agent]
      C -->|Read Rate Card & History| D[Tenant Knowledge Graph DB]
      C -->|Check Availability| E[Operations Agent]
      C -->|Draft Custom Quote| F[Action Required Queue]
      F --> G[Mobile App Feed 375px]
      G -->|1-Tap Send| H[Omnichannel Dispatcher]
      H --> I[Customer Proposal Link]
      I -->|Accept & Pay Deposit| J[Stripe Checkout]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Customer Intake Flow:** A conversational form or direct DM where the customer describes their unique problem or project needs.
  - **Owner Dashboard (Feed):** The owner receives a unified feed card: "Custom Request from John (Plumbing)".
  - **Interaction:** Tapping the card reveals the original request on top, and an AI-generated draft proposal below. The proposal includes itemized pricing, generated from the owner's internal rate card (e.g., $85/hr for labor, estimated 3 hours).
  - **Action:** A prominent "Send Quote" button, an "Edit Line Items" button, and a "Regenerate" button. The quote itself is a translucent glass card.

  ### AI Agent Integration Points
  - **The Negotiator Agent:** Triggered by new custom intake requests. Uses a system prompt focused on the specific service vertical (e.g., Handyman) and queries the tenant's past successful quotes for pricing consistency.
  - **Operations Agent Integration:** The Negotiator checks with the Operations agent to ensure there is actually calendar space or inventory before proposing a timeline.

  ### Key Design Decisions
  - **Internal Rate Card vs Public Pricing:** The owner can define internal pricing heuristics (e.g., "Minimum charge $150", "Add 20% for rush jobs") that the Negotiator uses invisibly.
  - **Human-in-the-Loop:** The Negotiator drafts the proposal, but the owner must tap "Send" to finalize it, maintaining control over custom pricing.

  ## Implementation Prompt
  **User-Facing Outcome:** As an owner (like Carlos), when a customer messages "I have a broken window and need it fixed today," I receive a push notification. I open the app and see a pre-written quote for $200 (Rush Fee + Standard Labor + Materials Estimate). I tap "Send," and the customer gets a link to approve and pay the deposit.

  **CUJ & Acceptance Criteria:**
  1. Create a `RateCard` entity in the database that stores pricing rules for a tenant.
  2. Implement the `The Negotiator Agent` capable of receiving unstructured text requests and outputting a structured `Proposal` object (line items, total, timeline).
  3. Develop the Mobile-First UI (375px) for the owner to review and edit the draft proposal in the Action Required feed.
  4. Write Playwright E2E tests: Simulate a customer request, verify the Negotiation agent generates a quote based on the tenant's rate card, and have the owner approve it via the UI.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
