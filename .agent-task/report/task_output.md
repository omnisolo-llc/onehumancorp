issue_title: "Architecture: Agentic Proposal & Quote Generation Engine"
issue_description: |
  ## Problem Statement
  Service-based and custom-order business owners (e.g., Nora the Agency Principal, Carlos the Handyman, Maya the Baker) spend a disproportionate amount of time turning incoming customer demand into actionable quotes and proposals. Traditional tools (HoneyBook, HubSpot, PandaDoc) require manual data entry, template selection, and pricing calculation. For a solopreneur or small operator, this manual friction leads to delayed responses and lost revenue. They need an assistant that instantly translates an unstructured customer request (e.g., an Instagram DM saying "How much for a 3-tier vegan wedding cake next month?") into a ready-to-send, accurate quote.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **HoneyBook / Dubsado:** Excellent at client portals and templates, but fundamentally passive. The owner must log in, build the quote, and send it. They lack RAG-based context awareness to draft the quote automatically.
  - **HubSpot / Salesforce:** Enterprise-grade CPQ (Configure, Price, Quote) is highly structured but requires a dedicated sales operations team to manage pricing rules and product catalogs. Completely unsuitable for Carlos or Maya.
  - **Shopify B2B / Draft Orders:** Allows creating a draft order and emailing an invoice, but lacks narrative proposal capabilities (e.g., scope of work, project milestones) needed by service professionals like Nora.
  - **OHC Opportunity:** Leverage our "Sales Assistant" agent. When an unstructured inquiry arrives via the unified inbox or intake form, the agent cross-references the tenant's product/service catalog, pricing rules, and availability. It then generates a structured proposal with line items, a narrative scope, and an integrated deposit payment link. The owner simply receives an Action Card in their feed: "Approve Quote for Sarah".

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Inquiry: DM/Email/Form] -->|Ingestion| B(Work Triage Agent)
      B --> C{Intent Classification}
      C -->|Quote Request| D[The Sales Assistant]
      D -->|Query Catalog & Pricing| E[(PostgreSQL: Tenant Catalog & Rules)]
      D -->|Query Calendar| F[(PostgreSQL: Availability)]
      D -->|Generate Draft| G[Quote/Proposal RAG Engine]
      G --> H[Draft Quote Record Created]
      H --> I[Action Card Push to Owner Feed]
      I --> J{Owner Review on Mobile}
      J -->|1-Tap Approve| K[Dispatch via Omnichannel]
      K --> L[Customer clicks link -> Stripe Checkout]
  ```

  ### Mobile UX Flow (375px First)
  - **Work Feed (Home):** A new Action Card appears: "Draft Quote Ready: 3-Tier Vegan Cake for Sarah ($350)".
  - **Card Interaction:** Tapping the card expands a mobile-optimized summary.
    - Top: Customer Context (Sarah, new lead).
    - Middle: AI-drafted message ("Hi Sarah, I'd love to bake this for you! Here is the estimate...").
    - Bottom: Line items (3-Tier Cake - $300, Vegan Surcharge - $50) and Deposit requirement ($175).
  - **Action Buttons:** A prominent primary button "Approve & Send", a secondary "Edit Items", and a tertiary "Decline".
  - **Visual Design:** Utilizes OHC Premium Tokens (translucent glass styling, clear typographic hierarchy). The quote preview itself should look like a beautiful, consumer-grade Apple Pay summary sheet, avoiding complex spreadsheet-like views on mobile.

  ### AI Agent Integration Points
  - **Work Triage Agent:** Identifies that a message is asking for a price or proposal.
  - **The Sales Assistant:** Instructed via its system prompt to act as an estimator. It uses a defined tool `generate_quote_draft(tenant_id, customer_id, line_items, scope_text, deposit_percentage)` to output structured data rather than just text.
  - **Memory/Context:** The agent pulls from the tenant's memory bank (e.g., previous similar projects, base hourly rate, material costs) to inform the pricing.

  ### Key Design Decisions
  - **Structured Output over Plain Text:** The LLM must output structured JSON for line items so they can map directly to a Stripe Payment Link/Invoice, rather than just sending a text message with a price.
  - **Owner in the Loop (HitL):** Quotes represent a legally binding offer and financial commitment. The system MUST require explicit owner approval via the Action Card before sending. No autonomous sending of quotes.
  - **Deposit-First Architecture:** Built-in support for requiring a % deposit to accept the quote, reflecting the reality of custom work and service bookings.

  ## Implementation Prompt
  **User-Facing Outcome:** As an owner (like Carlos or Maya), when a customer asks for a price on a custom job, I want my assistant to instantly draft a professional quote with line items and a deposit link, so I can review and send it from my phone with one tap while on the go.

  **CUJ & Acceptance Criteria:**
  1. A simulated customer message requesting a quote is processed by the system.
  2. The Sales Assistant agent is invoked, analyzes the request, queries the tenant's mock catalog, and uses the `generate_quote_draft` tool to create a structured `Proposal` record in the database.
  3. An Action Card appears in the owner's mobile UI feed displaying the draft quote.
  4. The owner taps "Approve & Send".
  5. The system transitions the Proposal state to "Sent" and triggers the omnichannel dispatcher to send a payment link to the customer.
  6. Provide Playwright E2E tests: A user logs in, navigates to their feed, sees the draft quote card, taps "Approve," and verifies the quote state updates and a simulated message is dispatched.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
