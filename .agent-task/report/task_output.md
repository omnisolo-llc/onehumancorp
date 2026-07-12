issue_title: "Architecture Design: AI-Automated Dynamic Quote Generator (The Closer Agent)"
issue_description: |
  ## Problem Statement
  Service-based owners and operators (e.g., Carlos the handyman, Nora the agency principal) often lose leads because drafting professional quotes, estimates, and proposals requires context switching, manual pricing lookup, and time spent formatting documents. Traditional CRMs (HubSpot) or invoicing tools (QuickBooks, FreshBooks) require manual data entry and are too complex for mobile-first solopreneurs. They need a system that translates a quick customer DM or voice note into a ready-to-send proposal with a built-in deposit link.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **HubSpot / Salesforce:** Enterprise CRMs have proposal generators, but they require heavy setup, manual pipeline management, and desktop usage. They are not built for mobile-first solopreneurs.
  - **Jobber / Housecall Pro:** Great for field services, but require the user to manually select line items, calculate totals, and create the PDF. They lack an AI assistant that reads natural language to build the quote.
  - **Shopify / Wix:** Primarily built for static product catalogs. They lack native, dynamic service-quote generation and negotiation flows.
  - **OHC Opportunity:** Leverage the "Closer Agent" to bridge the gap between intake and revenue. The agent reads the conversation (e.g., "Customer needs 3 ceiling fans installed"), checks the owner's standard service catalog for pricing, drafts a line-item quote, attaches a Stripe deposit link, and presents it to the owner for 1-tap approval on a 375px mobile screen.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request via DM/Inbox] --> B(Work Triage Gateway)
      B --> C{Closer Agent}
      C -->|Read Pricing| D[(Tenant Catalog & Pricing DB)]
      C -->|Draft Quote| E[Quote Draft Generation]
      E --> F[Stripe Integration: Draft Payment Link]
      F --> G[Action Required Queue]
      G --> H[Owner Mobile App Feed 375px]
      H -->|1-Tap Approve| I[Quote Dispatch via Email/SMS]
      I --> J[Customer Payment]
      J --> K[Webhook Updates Booking/Task]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "Action Required: Approve Quote for John (3 Ceiling Fans)".
  - **Interaction:** Tapping the card opens the drafted quote. The UI displays clean, translucent Glassmorphism cards showing:
    - **Context:** Brief summary of the request.
    - **Line Items:** AI-extracted items and prices (e.g., 3x Fan Installation @ $150).
    - **Total & Deposit:** Total $450, requiring a $100 deposit to book.
  - **Action:** A sticky primary bottom button "Send Quote ($450)". Secondary buttons to "Edit Line Items" or "Regenerate".
  - **Visual Design:** Mobile-first Apple-style unified dashboard layout. Focus on the final actionable output.

  ### AI Agent Integration Points
  - **Closer Agent:** Triggered when the Work Triage system identifies a commercial intent. Uses RAG against the tenant's past quotes and service pricing. Constructs structured JSON line items and standard terms.
  - **Finance Assistant:** Coordinates with Stripe to provision an idempotent Payment Link or Checkout Session tied to the quote, handling the deposit requirement.

  ### Key Design Decisions
  - **Structured JSON Output:** The Closer Agent must output structured data (line items, quantities, prices) rather than just a text email, enabling the UI to render an interactive cart-like quote and link directly to Stripe.
  - **Owner Approval Gate:** Quotes are high-stakes. The AI drafts the quote, but it is placed in the "Action Required" feed. It is never sent autonomously without owner approval.
  - **Multi-Tenant Isolation:** Pricing data and Stripe API keys must be strictly isolated using row-level security and `tenant_id` claims.

  ## Implementation Prompt
  **User-Facing Outcome:** As an owner (Carlos), when a customer messages me asking for a service, I open OHC to find a fully calculated quote waiting in my feed. I review the line items on my phone, tap "Approve", and the customer receives a professional web-link quote with a Stripe deposit button.
  **CUJ & Acceptance Criteria:**
  1. A simulated customer request is ingested indicating service needs.
  2. The Closer Agent parses the intent, queries the tenant's pricing table, and creates a `Quote` record with line items and a pending status.
  3. The system generates a Stripe Payment Intent or Checkout Session for the required deposit amount.
  4. The Quote appears in the owner's mobile feed as "Action Required".
  5. Provide Playwright E2E tests: A user logs in, taps "Approve" on the drafted quote card, and the UI transitions the quote to "Sent", verifying that the mock external message dispatch was triggered with the valid payment link.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
