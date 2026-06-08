issue_title: "The Negotiator Agent: Autonomous Custom Quotes & Dynamic Pricing"
issue_description: |
  # The Negotiator Agent: Autonomous Custom Quotes & Dynamic Pricing

  ## Problem Statement
  Service-based small business owners (e.g., Carlos the Field Service Owner, Nora the Agency Principal, Maya the Home Baker) lose highly qualified leads and critical revenue because they cannot respond instantly with accurate, negotiated quotes for custom requests. Manual estimation requires calculating materials, estimating time, checking calendar availability, and drafting proposals—often done at night or weekends. Traditional platforms like Shopify or Wix force rigid product variants or basic contact forms, providing no intelligent bridge between the customer's bespoke request and a final, payable invoice.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify & Wix:** Focus heavily on static product catalogs. Custom orders require manual back-and-forth communication or expensive, rigid third-party form builders that do not calculate real-time pricing dynamically.
  - **Jobber / ServiceTitan:** Excellent for field services but overly complex, expensive, and behave like heavy CRM systems rather than proactive assistants.
  - **HoneyBook / Dubsado:** Great proposal software for creatives, but lacks real-time, AI-driven quote generation based on an integrated catalog of materials and dynamic availability.
  - **OHC Opportunity:** Leverage our Agentic workflow to introduce "The Negotiator." When a customer submits a complex request (e.g., "I need a 3-tier vegan wedding cake for 150 people delivered next Saturday"), The Negotiator instantly parses the intent, queries the materials database, checks production availability (via The Manager/Operations Agent), and drafts a professional, dynamically priced quote for the owner's 1-tap approval.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request - Web/DM] -->|Webhook| B(Omnichannel Gateway)
      B --> C{The Negotiator Agent}
      C -->|Check Availability| D[Operations Agent / Calendar]
      C -->|Query Costs| E[PostgreSQL Catalog/Materials]
      C -->|Analyze History| F[Customer Graph DB]
      C -->|Draft Quote| G[Quote Engine]
      G --> H[Action Required Queue]
      H --> I[Mobile App Feed 375px]
      I -->|1-Tap Approve| J[Stripe Payment Link / Proposal]
      J --> K[Customer Delivery]
  ```

  ### Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** A priority card appears: "New Custom Request: 3-Tier Vegan Wedding Cake (Maya's Bakery)".
  - **Interaction:** Tapping the card reveals a split view.
    - Top half: The customer's original message and CRM context.
    - Bottom half: The Agent-drafted proposal outlining itemized costs (materials, labor, delivery fee), a proposed completion date, and a final price of $450.
  - **Action:** A prominent primary button "Approve & Send Quote" and a secondary "Edit Details" button.
  - **Visual Design:** Clean glassmorphism layout, strong typography, ensuring all numerical estimates are highly readable. Native mobile keyboard integration if the owner decides to edit the price or line items.

  ### AI Agent Integration Points
  - **The Negotiator Agent (Sales/Finance):** Uses intent parsing and RAG against the tenant's pricing matrix, past accepted quotes, and material costs to generate highly accurate estimates.
  - **The Manager (Operations):** Consulted by The Negotiator to ensure there is physical capacity (e.g., enough oven time or delivery slots) before offering a date in the quote.

  ### Key Design Decisions
  - **Instant Estimation, Manual Approval:** The system should never send a binding financial quote without the owner's explicit 1-tap approval, maintaining trust and safety.
  - **Dynamic Margin Protection:** The Agent automatically factors in desired profit margins based on the tenant's global settings, protecting the business from underpricing custom work.
  - **Integrated Payment:** The approved quote instantly generates a Stripe Payment Link for the required deposit, seamlessly moving the customer to checkout.

  ## Implementation Prompt
  **User-Facing Outcome:** As a business owner, when a customer submits a complex custom order request, I open my OHC app to find a fully calculated, itemized quote already drafted. I review the line items, tap "Approve & Send," and the customer receives a professional proposal with a deposit payment link in seconds.

  **CUJ & Acceptance Criteria:**
  1. A custom request is ingested via an API endpoint or mocked form submission.
  2. The Negotiator Agent correctly parses the natural language request and maps it to underlying catalog items/materials.
  3. The Agent queries the Operations Agent/Calendar to verify capacity for the requested date.
  4. A Draft Quote object is created, containing itemized costs and a final price, and placed in the mobile feed.
  5. Provide Playwright E2E tests: A user logs in, navigates to the 375px feed, views the drafted quote, taps "Approve & Send," and the system generates a simulated Stripe Payment Link for the customer.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
