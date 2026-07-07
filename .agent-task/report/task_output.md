issue_title: "Implement Universal Multi-Currency and Localized Pricing Engine"
issue_description: |
  # Universal Multi-Currency and Localized Pricing Engine

  ## Problem Statement
  Currently, OneHumanCorp (OHC) operates on the assumption of a single, fixed currency (typically USD) across all platform surfaces. This structural limitation actively excludes international owners and operators. When "Priya" (a boutique operator based in London) attempts to sell her clothing online to customers in the US and EU, she is unable to price her items in GBP while displaying localized EUR and USD prices to her foreign customers. For non-technical owners, manually calculating conversion rates or managing multiple storefronts for different regions is impossible. They need the AI Assistant to automatically handle currency localization, ensuring customers see prices in their native currency while the owner's revenue reporting remains perfectly reconciled in their home currency.

  ## Research Report
  - **Competitor Analysis**:
    - **Shopify**: Shopify Markets is a foundational primitive that handles multi-currency seamlessly. It dynamically detects the buyer's region, applies a FX conversion rate (often with a small spread), and presents localized pricing. Payouts are reconciled into the merchant's home currency.
    - **Stripe**: Handles multi-currency through `Presentment Currency` vs `Settlement Currency`. They allow creating Prices with multiple localized amounts or automatically converting via real-time exchange rates.
    - **Wix & Squarespace**: Both offer multi-currency toggle options, but often require third-party apps or higher-tier plans.
  - **The OHC Differentiator**: OHC's AI Finance & Decision Assistant can completely abstract this complexity. Instead of forcing the owner to manually configure FX spreads or localization rules, the Assistant simply asks: "Do you want to sell internationally?" and auto-configures the presentment currencies while guaranteeing the owner's daily summary remains in their unified base currency.

  ## Design Doc

  ### Architecture Diagram

  #### Sequence Diagram
  ```mermaid
  sequenceDiagram
      actor Buyer as Buyer (France)
      participant Edge as Edge Router
      participant Pricing as Localized Pricing Engine
      participant FX as FX Exchange Cache
      participant API as Storefront API
      actor Owner as Owner (Priya - UK)
      participant AI as Finance AI Assistant

      Buyer->>Edge: Visits Storefront
      Edge->>Pricing: Detects EUR Region, Requests Price
      Pricing->>FX: Fetch GBP to EUR Rate
      FX-->>Pricing: Return Rate (e.g. 1.18)
      Pricing-->>API: Return EUR Price
      API-->>Buyer: Displays €45.00

      Owner->>AI: Checks Daily Summary
      AI->>Pricing: Fetch Reconciled Revenue
      Pricing-->>AI: Returns GBP Total
      AI-->>Owner: Reports £1000 Total
  ```

  #### Entity-Relationship Diagram
  ```mermaid
  erDiagram
      TENANT {
          uuid id PK
          string name
          char(3) base_currency "e.g., 'GBP'"
          boolean international_sales_enabled
      }
      PRODUCT {
          uuid id PK
          uuid tenant_id FK
          string name
          bigint base_price_cents
      }
      CURRENCY_EXCHANGE_RATE {
          char(3) source_currency PK
          char(3) target_currency PK
          decimal rate
          timestamp last_updated
      }
      ORDER {
          uuid id PK
          uuid tenant_id FK
          uuid product_id FK
          char(3) presentment_currency "e.g., 'EUR'"
          bigint presentment_amount_cents
          char(3) settlement_currency "e.g., 'GBP'"
          bigint settlement_amount_cents
          decimal exchange_rate_applied
      }

      TENANT ||--o{ PRODUCT : owns
      TENANT ||--o{ ORDER : processes
      PRODUCT ||--o{ ORDER : includes
  ```

  ### Mobile UX Flow (375px First)
  1. **Settings / Commerce Tab**:
     - A clean, UniFi-style card titled "International Sales".
     - Toggle: "Accept payments in other currencies".
     - The AI Assistant adds a conversational prompt: "I'll automatically convert foreign payments to your base currency (GBP) so you don't have to worry about exchange rates."
  2. **Product Edit Screen**:
     - Price input displays the owner's base currency symbol (e.g., `£` fixed on the left).
     - An "Advanced Settings" drawer reveals explicit price overrides for specific regions (e.g., setting a static USD price instead of floating FX).
  3. **Daily Summary Feed**:
     - Revenue is always presented in the owner's base currency.
     - A subtle info icon indicates: "Includes £150 converted from EUR and USD sales."

  ### AI Agent Integration Points
  - **Finance & Decision Assistant**: Must be aware of both presentment and settlement currencies when aggregating daily revenue. It needs to explain FX variances (e.g., "Your revenue grew 5% today, partially due to a stronger Dollar against the Pound.").
  - **Sales & Revenue Assistant**: When drafting proposals or quotes for international clients (e.g., for Nora the Agency Principal), the agent automatically drafts the proposal using the client's local currency to increase conversion rates.

  ### Key Design Decisions
  - **Zero Manual Math**: The owner should never have to manually input exchange rates. We will rely on an automated FX caching service synced daily.
  - **Clear Settlement Expectations**: The platform must guarantee visual consistency—what the buyer sees is what they pay, and the owner always knows exactly what they will receive in their base currency after conversion.
  - **Strict Multi-Tenant Isolation**: Currency preferences (Base Currency) are strictly scoped to the `tenant` level.

  ## Implementation Prompt
  **Goal**: Implement a robust Multi-Currency Pricing Engine that allows tenants to configure a base currency and automatically localize pricing for international buyers.

  **Acceptance Criteria**:
  1. An owner can set a "Base Currency" for their workspace via the mobile-first UI.
  2. The pricing engine supports presentment in at least three major currencies (USD, EUR, GBP) via a dynamic exchange rate layer.
  3. The daily revenue summary AI correctly aggregates multi-currency sales back into the owner's base currency for reporting.
  4. The solution is fully responsive down to 375px, maintaining the premium translucent UI design.
  5. Playwright E2E tests are implemented: A test logs in as a tenant, sets GBP as the base currency, creates a £10 product, simulates a buyer viewing it in EUR, and verifies the converted price is displayed correctly without breaking the UI.
  6. Ensure 100% unit test coverage for the new pricing logic.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
