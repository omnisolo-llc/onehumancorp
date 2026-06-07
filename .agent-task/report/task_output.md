issue_title: "AI-Powered Autonomous Multi-Currency & Cross-Border Localization Engine"
issue_description: |
  # Research Report: AI-Powered Autonomous Multi-Currency & Cross-Border Localization Engine

  ## Problem Statement
  Small business owners and creators (like Priya the Boutique Operator or Leo the Creator) often struggle to expand their business globally because managing multiple currencies, dynamic exchange rates, localized pricing strategies, and regional tax/shipping rules is overwhelmingly complex. Existing platforms like Shopify require expensive third-party apps or premium "Markets" tiers, while simpler builders like Wix or GoDaddy offer rudimentary, manual currency conversion that frustrates international buyers. The gap is the lack of an *autonomous* system that handles cross-border localization invisibly, allowing an owner to seamlessly sell to anyone, anywhere, without becoming an international finance expert.

  ## Research Report (Track 1)
  - **Competitor Landscape**:
    - *Shopify*: Strong with "Shopify Markets," but it's a premium feature that still requires manual configuration of price rounding, regional shipping zones, and tax liabilities.
    - *Wix/Squarespace*: Basic currency toggles for display, but often checkout still happens in the base currency, confusing buyers.
    - *Stripe*: Offers multi-currency presentment, but the integration into the storefront UI and catalog pricing strategy is left to the developer/merchant.
  - **The OHC Opportunity**: Integrate a natively autonomous cross-border engine where the "Operations Agent" and "Finance Agent" coordinate to dynamically present localized pricing, handle exchange rate fluctuations within safe margins, and coordinate localized shipping/tax—all without the merchant configuring complex rules.

  ## Design Doc (Track 2 & 3)

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User as Global Buyer (Mobile)
      participant Edge as OHC Edge Server (Cloudflare)
      participant Core as OHC Core App (Axum/PostgreSQL)
      participant FinAgent as Finance Agent
      participant Stripe as Stripe API

      User->>Edge: Request Storefront (IP/Geo detected)
      Edge->>Core: Fetch product prices (Geo-context)
      Core->>FinAgent: Calculate localized price (Current FX + Margin Rule)
      FinAgent-->>Core: Localized Prices & Currency (e.g., EUR)
      Core-->>Edge: HTML/JSON with EUR pricing
      Edge-->>User: Storefront displayed in EUR

      User->>Core: Initiate Checkout (Tap-to-Pay or Web)
      Core->>Stripe: Create PaymentIntent (Currency: EUR)
      Stripe-->>Core: Session ID
      Core-->>User: Secure Checkout UI
  ```

  ### Data Model Enhancements
  - **`TenantConfig`**: Add `accepted_currencies` and `auto_localize_pricing` (boolean) fields.
  - **`Product`**: Base `price_cents` and `currency` remain the source of truth.
  - **`LocalizedPricingCache` (Redis)**: Cache daily/hourly exchange rates and computed prices per tenant to ensure sub-100ms response times at the edge.

  ### Mobile UX Flow (375px First)
  1. **Buyer View**: When a buyer from France visits Priya's US-based store on their phone, the prices instantly show in EUR (e.g., €45.00 instead of $48.23, utilizing psychological price rounding). A subtle, dismissible bottom toast says, "Prices localized for your region."
  2. **Owner View (OHC App)**: Priya sees a unified dashboard. A new "Global Reach" card (Ubiquiti-style clean layout, translucent glass) shows sales mapped by region. If exchange rates shift drastically, the Finance Agent pushes a notification: "GBP has dropped 5%. I've adjusted your UK margins to protect profitability. Tap to review."

  ### AI Agent Integration
  - **Finance Agent ("The Accountant")**: Monitors FX rates, applies psychological price rounding (e.g., making €48.23 into €49.00), and ensures cross-border transaction fees don't eat into the merchant's base margin.

  ## Implementation Prompt
  **Feature Name**: Autonomous Cross-Border Engine
  **Target Persona**: Priya the Boutique Operator
  **Outcome**: Priya's online store automatically displays and processes payments in the buyer's local currency, with AI-managed exchange rates and margin protection, requiring zero manual configuration from Priya.

  **Next Actions for Engineering (Implementer Agent)**:
  1. **Database & Core**: Extend the `products` and tenant settings in PostgreSQL to support a multi-currency pricing strategy.
  2. **Service Layer**: Implement a real-time (or Redis-cached) currency conversion utility that the Finance Agent can utilize to compute localized prices dynamically.
  3. **Frontend (Tauri/Next.js)**: Update the storefront display components to seamlessly render localized prices based on the user's IP/Geo header, ensuring UI stability on 375px viewports.
  4. **Stripe Integration**: Ensure the checkout flow correctly passes the localized currency and amount to Stripe PaymentIntents/Sessions.
  5. **Verification**: Write E2E Playwright tests that mock a buyer visiting from a different region, verifying that the storefront displays the correct currency and the checkout session is created in that currency.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
