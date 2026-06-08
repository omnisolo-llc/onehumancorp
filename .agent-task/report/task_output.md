issue_title: "Implement High-Scale Multi-Currency Architecture for OHC Storefronts"
issue_description: |
  # High-Scale Multi-Currency Architecture Research Report

  ## Problem Statement
  Currently, OHC platforms are built around single-currency environments. For business personas like Priya (Boutique Operator) who wishes to expand her online demand globally, restricting the storefront and checkout flows to a single currency creates extreme friction and lost sales. Competitor tools like Shopify and Stripe seamlessly support dynamic currency conversion, localized pricing, and regional formatting, making this a critical gap in OHC.

  ## Research Report
  - **Competitor Analysis:** Shopify handles multi-currency via native global edge infrastructure that caches localized pricing. Stripe provides extensive API support for dynamically converting product prices based on user locale, capturing payments, and handling settlement in the merchant's base currency.
  - **Persona Focus (Priya):** Wants online demand without losing control of operations. She requires her online inventory to display in the local currency of her visitors, improving conversion rates without adding operational complexity to her daily ledger and performance summaries.

  ## Design Doc
  - **Architecture Concept:**
    - Introduce a centralized `CurrencyConfig` data entity that links to tenant configurations (e.g., base currency, supported currencies).
    - Provide real-time/cached exchange rates, updating via a reliable external provider or daily sync.
    - Implement a `PriceDisplayService` to intercept product catalogs and localized prices at the edge before serving the UI.

    ```mermaid
    erDiagram
        TenantSettings ||--o{ CurrencyConfig : "has"
        CurrencyConfig {
            string base_currency
            json supported_currencies
        }
        ExchangeRateCache {
            string from_currency
            string to_currency
            float rate
            datetime last_updated
        }
        ProductCatalog ||--o{ PriceDisplayService : "requests localized pricing"
        PriceDisplayService ||--o{ ExchangeRateCache : "fetches current rates"
    ```

    - **Data Model:**
      - `TenantSettings` (enhanced to support base_currency).
      - `ExchangeRateCache` (Currency Pair, Rate, LastUpdated).
      - Ensure multi-tenant isolation with Strict RLS.
    - **Mobile UX Flow (375px first):**
      - Users browse the storefront on their phones.
      - A lightweight, non-intrusive bottom sheet or profile dropdown allows them to switch currency, instantly updating catalog prices without page reload.
      - Cart and Checkout transparently reflect the localized pricing.
    - **AI Agent Integration:**
      - **Finance Assistant:** Flags significant exchange rate fluctuations that may affect margins and suggests adjusting base prices.
      - **Customer Assistant:** Automatically identifies the customer's region from initial inquiries and defaults chat-based quotes to their local currency.

  ## Implementation Prompt
  Implement the foundational backend data model and API endpoints necessary to support multi-currency pricing for OHC. Define the `CurrencyConfig` schema, enable dynamic price resolution based on user locale, and integrate this seamlessly into the existing product catalog and checkout endpoints. The system must support real-time fallback to the base currency if conversion fails. Ensure all changes are covered by 100% unit tests and write Playwright E2E tests validating the storefront catalog viewing and checkout processes in at least two different currencies. Provide clear, mobile-friendly UX elements for currency switching.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
