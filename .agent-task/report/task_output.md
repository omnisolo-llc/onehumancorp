issue_title: "OHC Multi-Currency & Localized Checkout Engine"
issue_description: |
  # Research Report: OHC Multi-Currency & Localized Checkout Engine

  ## Problem Statement
  Small business owners and creators (like Priya the boutique owner or Leo the tutor) who want to sell internationally or accept payments from tourists face a complex setup. They need to configure exchange rates, local payment methods, and handle currency conversions. Existing platforms often require premium tiers or third-party apps to handle true multi-currency with localized payment methods (e.g., iDEAL in Netherlands, Alipay in Asia), creating friction and abandoning international sales. The owner wants a system that "just works" globally without them having to become a foreign exchange expert.

  ## Research Report
  - **Market Context**: E-commerce is increasingly borderless, even for SMBs. Customers expect to see prices and pay in their local currency.
  - **Competitor Gaps**:
    - *Shopify*: Offers multi-currency through Shopify Markets, but it's complex to configure and often requires Shopify Payments (not available everywhere).
    - *Wix/Squarespace*: Basic multi-currency display, but checkout often reverts to the store's base currency, causing confusion and drop-offs.
  - **The OHC Opportunity**: By deeply integrating with Stripe's localized pricing and payment methods APIs, OHC can offer an "always local" checkout experience. The Finance Agent can handle the complexity of exchange rate fluctuations and suggest optimal pricing strategies for different regions.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      CUSTOMER ||--o{ CHECKOUT_SESSION : "initiates"
      CHECKOUT_SESSION }|--|| EDGE_PRICING_SERVICE : "fetches localized price"
      EDGE_PRICING_SERVICE ||--|{ CACHE : "reads exchange rates"
      CHECKOUT_SESSION ||--|| LEDGER : "records multi-currency transaction"
      LEDGER ||--o{ FINANCE_AGENT : "monitors margins"
  ```

  ### Mobile UX Flow & Wireframes (375px)
  1. **Customer View**: A seamless product page that auto-detects location and displays prices in the local currency. Checkout offers local payment methods prominently.
  2. **Owner View (Dashboard)**: The unified feed shows international sales translated to the base currency, with a clear breakdown of exchange fees and the Finance Agent's margin insights.

  ### AI Agent Integration Points
  - **Finance Agent**: Monitors exchange rates and alerts the owner if profit margins are slipping due to currency fluctuations. It can proactively suggest adjusting regional prices.

  ### Key Design Decisions
  - **Edge-cached Pricing**: We will cache exchange rates at the edge to ensure product pages load blazingly fast globally, without hitting the core database on every view.
  - **Dual-Currency Ledger**: The ledger must track both presentment and settlement currencies natively, avoiding "app tax" patches on top of a single-currency core.

  ## Implementation Prompt
  **Feature Name**: OHC Multi-Currency Checkout Engine
  **Target Persona**: Priya the Boutique Operator
  **Outcome**: Priya's online store automatically displays prices in EUR for European visitors and allows them to pay with local methods like iDEAL or Bancontact, while Priya receives payouts in USD. The Finance Agent alerts her if the EUR/USD exchange rate impacts her margins.

  **Acceptance Criteria**:
  1. Implement a low-latency caching mechanism for localized pricing rules and exchange rates.
  2. Ensure the checkout flow dynamically displays the correct local currency and payment methods for the user.
  3. Ensure the core ledger tracks both presentment currency (what the customer paid) and settlement currency (what the owner receives).
  4. Build a background job for the Finance Agent to alert the owner of significant exchange rate fluctuations affecting margins.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
