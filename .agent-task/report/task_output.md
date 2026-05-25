issue_title: "Implement Global Multi-Currency Treasury and Localization Mesh"
issue_description: |
  # Architecture: Global Multi-Currency Treasury and Localization Mesh

  ## Problem Statement
  Small business owners like Priya (boutique owner) and Fatima (food cart) increasingly operate in diverse local communities or sell online across borders. Currently, if Priya wants to sell her boutique clothing online to customers in Canada and the UK, she faces a major hurdle: pricing, currencies, tax calculation, and shipping rules must all be manually configured per region. If Fatima has customers who prefer to see menus and receipts in Spanish or Arabic, there's no seamless way to dynamically localize the storefront without managing entirely separate menus or websites. For a non-technical business owner, the friction of internationalizing or localizing their business is paralyzing. They want to flip a switch that says "Sell Globally" and have the system invisibly handle currency conversion, localized checkout, cross-border shipping, and multilingual UI without any manual spreadsheet work or reading confusing tax manuals.

  ## Research Report
  The most successful modern commerce platforms abstract away cross-border complexity:
  - **Shopify Markets**: Provides a unified interface to manage localized storefronts, automatic currency conversion, calculated duties/import taxes, and localized domains. It dynamically detects the buyer's region and adjusts the experience, but still requires the merchant to configure market specific pricing rules and shipping zones, which can be overwhelming.
  - **Stripe Elements & Checkout**: Excels at localized payment methods (e.g., iDEAL in the Netherlands, Alipay in China, Klarna in Europe) and automatic dynamic currency conversion. However, it's primarily a payment layer, lacking full storefront and catalog localization features out-of-the-box.
  - **Wix Multilingual**: Allows for site translation, but currency conversion and localized tax/shipping rules are often handled by disparate third-party apps, breaking the unified seamless experience for the merchant.

  **OneHumanCorp Gap**: OHC currently lacks a unified, invisible localization engine. We need a "Global Multi-Currency Treasury and Localization Mesh" that automatically localizes the storefront (language, currency, taxes, localized payment methods) based on the buyer's context, while the merchant manages a single, global catalog in their base currency and language. The AI should handle all translations, tax calculations, and FX volatility risks behind the scenes.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      MERCHANT_CATALOG ||--o{ LOCALIZED_CATALOG_CACHE : "hydrates"
      MERCHANT_CATALOG {
          string id
          string merchant_id
          string base_currency
          string base_language
          float base_price
      }
      LOCALIZED_CATALOG_CACHE {
          string id
          string target_region
          string target_currency
          string target_language
          float localized_price
      }
      BUYER_SESSION ||--o{ EDGE_ROUTER : "initiates"
      EDGE_ROUTER ||--o{ LOCALIZATION_ENGINE : "requests context"
      LOCALIZATION_ENGINE ||--o{ FX_RATE_SERVICE : "fetches real-time rates"
      LOCALIZATION_ENGINE ||--o{ AI_TRANSLATION_DEPT : "requests translation"
      LOCALIZATION_ENGINE ||--o{ TAX_COMPLIANCE_DEPT : "calculates duties/taxes"
      EDGE_ROUTER ||--o{ LOCALIZED_CATALOG_CACHE : "serves"
      CHECKOUT_SESSION ||--o{ TREASURY_LEDGER : "records transaction"
      TREASURY_LEDGER {
          string id
          string merchant_id
          string base_currency
          string transaction_currency
          float transaction_amount
          float settlement_amount
          float fx_rate_applied
      }
  ```

  ### Mobile-First UI Flow (375px)
  1. **Settings > Global Selling**: The merchant taps a single toggle switch: "Enable Global Customers".
  2. **AI Configuration Modal**: An AI conversational sheet slides up: "I see you're based in the US. I will automatically translate your storefront and convert prices for international visitors. Should I automatically calculate and include cross-border shipping and import taxes in the final price?" (Yes / No).
  3. **Buyer Experience**: A customer in France visits the storefront on their phone.
      - The **Edge Router** detects the IP/Locale.
      - The storefront instantly loads in French.
      - Prices are displayed in EUR (calculated from the base USD price + real-time FX buffer).
      - At checkout, localized payment methods (e.g., Cartes Bancaires) are presented.

  ### AI Integration Points
  - **AI Translation Dept**: Background agents monitor the `MERCHANT_CATALOG`. Upon any creation or update of a product title/description, the agents proactively translate the content into supported languages and populate the `LOCALIZED_CATALOG_CACHE`.
  - **Finance Dept (FX & Tax)**: Agents monitor global exchange rates to buffer prices against volatility. They calculate estimated duties and taxes dynamically based on the merchant's origin and buyer's destination, ensuring the checkout price is the final landed cost.
  - **Operations Dept**: Automatically updates shipping transit time estimates based on cross-border logistics data without the merchant needing to manually define shipping zones.

  ### Zero Trust & Security / Multi-tenancy
  - **Strict Multi-tenant Isolation**: All data entities (`MERCHANT_CATALOG`, `LOCALIZED_CATALOG_CACHE`, `TREASURY_LEDGER`) enforce a strict `merchant_id` boundary. A dedicated Multi-tenant Data Policy engine validates all database queries to guarantee no leakage between stores.
  - **SPIFFE/SPIRE Identity**: Communication between the `EDGE_ROUTER`, `LOCALIZATION_ENGINE`, and background AI agents relies on short-lived SPIFFE/SPIRE certificates. Only authorized AI Agents can write to the `LOCALIZED_CATALOG_CACHE`.

  ### Key Design Decisions (Visual Excellence Mandate)
  - **Zero Configuration**: The entire multi-currency and localization setup must be hidden behind a single "Enable Global Customers" toggle. No manual tax tables, no shipping zone configurations.
  - **Translucent Glass Materials**: The localized checkout experience uses iOS/macOS style translucent bottom sheets to present localized payment methods natively.
  - **Edge-First Delivery**: All localized catalogs must be pre-rendered and cached at the edge (CDN) to guarantee sub-100ms load times globally, ensuring the storefront feels instantaneous regardless of the buyer's location.

  ## Implementation Prompt
  **Task**: Implement the Global Multi-Currency Treasury and Localization Mesh.
  **User Journey**: Maya (the baker) enables "Global Customers" in her settings. A customer in Japan visits her store, sees the site in Japanese, prices in JPY, and pays with an local Japanese payment method. Maya receives the payment in her native USD, completely shielded from the complexity of FX rates, Japanese taxes, or translation.
  **Acceptance Criteria**:
  1. Create the `LocalizationEngine` service that interfaces with the edge router to determine buyer context (Locale/Currency).
  2. Implement the `TreasuryLedger` data structures to track multi-currency transactions and automated FX settlements.
  3. Integrate the AI Translation and Finance agent protocols to asynchronously populate the `LOCALIZED_CATALOG_CACHE` upon any catalog updates.
  4. Ensure the merchant UI remains a single toggle, with all complexity managed invisibly.
  5. All storefront requests must be served from the edge cache, maintaining strict performance targets (<100ms latency).

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []