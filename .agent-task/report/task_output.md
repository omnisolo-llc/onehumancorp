issue_title: "Implement Universal Multi-Currency and Localization Engine"
issue_description: |
  # Title: Implement Universal Multi-Currency and Localization Engine

  ## Problem Statement
  For non-technical small business owners like Fatima (the food cart operator) and Maya (the baker), reaching a broader audience or simply operating comfortably in their native language shouldn't require complex site duplication, language plugins, or manual currency conversions. Fatima needs her dashboard and customer-facing pre-order site to effortlessly toggle between Arabic and English, handling Right-to-Left (RTL) text natively. When Maya receives an Instagram DM order from a tourist paying in Euros, she needs to quote and accept payment seamlessly without leaving her unified inbox. Currently, building a multilingual, multi-currency storefront on legacy platforms requires expensive add-ons, manual translation of every product variant, and clunky third-party currency switchers that fail the "grandmother test." They need an invisible localization engine that automatically translates catalogs, adapts currencies based on the buyer's locale, and localizes the entire merchant operating system.

  ## Research Report

  **Market Gap:**
  *   **Shopify:** Requires "Shopify Markets" which is complex to configure. Translating catalogs requires installing third-party apps like "Translate & Adapt" and managing separate translation strings.
  *   **Wix / Squarespace:** Multi-language setups require duplicating pages or using awkward plugins. Managing different currencies often requires separate stores or premium integrations.
  *   **Stripe:** Handles multi-currency well at the payment layer, but doesn't solve the UI/catalog presentation layer for the merchant seamlessly.

  **Competitive Analysis:**
  Existing solutions force the user to become a localization manager. OHC must provide an "Autonomous Localization Mesh" where the AI acts as a continuous translation and localization service. When Fatima adds a new menu item in Arabic, the Operations Agent instantly generates the English equivalent. When a buyer in Mexico views Priya's boutique, the catalog and checkout automatically display in Spanish and MXN, utilizing real-time exchange rate caching at the edge.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Merchant as Fatima (Mobile App)
      participant Core as OHC Core
      participant OpsAgent as Operations Agent
      participant Edge as Edge CDN
      participant Buyer as Customer (Browser)
      participant Payment as Unified Payment Gateway

      Merchant->>Core: Adds menu item in Arabic
      Core->>OpsAgent: Trigger translation background job
      OpsAgent->>Core: Saves English translation & RTL metadata
      Core->>Edge: Invalidates cache & pushes localized data
      Buyer->>Edge: Visits site from US IP
      Edge-->>Buyer: Serves English catalog in USD
      Buyer->>Payment: Pays in USD
      Payment-->>Merchant: Settles in local currency invisibly
  ```

  ### Mobile UX Flow (375px first) & UI Wireframes
  *   **Merchant Settings:** A clean Translucent Glass card: "What languages do your customers speak?" with pills for [English], [Arabic], [Spanish]. No talk of "i18n strings" or "locale codes."
  *   **Auto-Translation Magic:** When Fatima types a menu item like "شاورما دجاج" (Chicken Shawarma), a subtle inline indicator shows "Translating to English... Done."
  *   **Buyer Checkout:** The buyer sees a completely native experience. The pricing card automatically shows the buyer's local currency based on IP/browser locale, with a subtle info icon showing the exchange rate if necessary.

  ### AI Agent Integration Points
  *   **Operations AI Department (Localization Sub-Agent):** Monitors the event mesh for any `entity.created` or `entity.updated` events (products, policies, auto-replies) and automatically generates and caches translations for all active merchant languages.
  *   **Customer Success AI Department:** Detects the language of incoming messages in the unified inbox and automatically translates them for the merchant, allowing Maya to read a French DM in English and reply in English, while the AI sends the response in French.

  ### Key Design Decisions
  *   **Edge-Cached Translations:** To hit strict latency targets, all translated strings and currency display rules are pushed to the Edge. The core database is not queried for read-heavy buyer traffic.
  *   **Zero-Trust Identity Isolation:** The localized data is strictly scoped to the tenant. The translation agent uses context only from that specific merchant's brand voice to ensure accurate translations.
  *   **Native RTL Support:** The entire mobile-first UI framework must programmatically support LTR to RTL switching without layout breaking, crucial for Arabic and Hebrew merchants.

  ## Implementation Prompt

  **User-Facing Outcome:**
  Deploy an invisible localization and multi-currency engine. The merchant simply selects the languages their business supports, and all subsequent catalog entries, store policies, and customer communications are automatically translated and formatted correctly (including RTL support). Buyers automatically see the storefront in their preferred language and local currency.

  **Core User Journeys (CUJ):**
  1.  **Autonomous Catalog Translation:** The merchant creates a product in their native language. The system automatically creates and links translated versions without the user manually editing them.
  2.  **Cross-Border Checkout:** A buyer visits the storefront from a different country. The edge gateway serves the localized catalog and pricing, and handles the multi-currency checkout transparently.
  3.  **Multilingual Inbox:** An incoming customer message in a foreign language is auto-translated in the merchant's unified inbox.

  **Acceptance Criteria:**
  *   Mobile-first design (375px) using the macOS-style Translucent Glass and UniFi modular card system for language selection.
  *   Background translation must occur via the event mesh without blocking UI interactions.
  *   Edge caching must be implemented so localized reads do not hit the primary database.
  *   Full support for RTL layouts on both merchant and buyer interfaces.
  *   No technical i18n jargon exposed to the merchant.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []