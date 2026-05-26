issue_title: "Implement Invisible Multi-Currency & Localization Mesh"
issue_description: |
  # Title: Invisible Multi-Currency & Localization Mesh

  ## Problem Statement

  Small business owners like Leo (online music tutor) and Priya (boutique owner) are operating in an increasingly global market. Leo teaches students in Europe, Asia, and Latin America via Zoom. Priya ships her unique handmade jewelry internationally. However, selling globally introduces massive friction: customers see prices in foreign currencies (causing sticker shock and cart abandonment), merchants pay exorbitant foreign exchange (FX) fees, and manually configuring localized pricing, taxes, and payment methods (like iDEAL in the Netherlands or Alipay in China) is overwhelmingly complex.

  Current platforms like Shopify require complex "Markets" configurations, forcing the merchant to understand FX risks and manually adjust settings. Stripe requires developer intervention to implement local payment methods. OHC needs a "Zero-Config" mesh that automatically localizes the storefront for the buyer (currency, language, local payment methods) while seamlessly settling in the merchant's native currency, completely hiding the FX complexity and buffering volatility.

  ## Research Report

  We analyzed how leading platforms handle internationalization and localization for SMBs.

  ### Competitive Analysis

  | Platform | Multi-Currency Strategy | Strengths | Weaknesses (The OHC Opportunity) |
  |---|---|---|---|
  | Shopify | Shopify Markets | Comprehensive rules for pricing and localized catalogs. | High friction setup. Requires the merchant to manage FX risk, price rounding rules, and explicitly enable regions. |
  | Stripe | Local Payment Methods / Multi-Currency | Excellent developer APIs, instant FX conversion. | Developer-first. A small business owner cannot easily configure this without engineering help. |
  | Wix | Multi-Currency App | Simple toggle to add currencies. | Often just a visual converter; checkout still happens in the base currency, confusing buyers. |
  | **OHC (Target)** | **Invisible Multi-Currency Mesh** | **Zero-Config, Edge-Localized Pricing, Automatic Local PMs, FX Volatility Buffer** | **Must provide a seamless native currency settlement for the merchant while optimizing conversion for the global buyer.** |

  ### Key Architectural Findings

  *   **Edge Compute is Required:** To avoid high-latency database queries on every page load, geolocation and currency localization must happen at the Edge (CDN level).
  *   **FX Volatility:** Simply converting prices at the real-time spot rate leads to weird prices (e.g., $10.00 becomes €9.23). OHC needs an AI pricing engine that buffers FX volatility and applies "charming" rounding rules (e.g., €9.99) while guaranteeing the merchant their expected margin.
  *   **Ledger Complexity:** Transactions must be recorded in the buyer's currency, but the merchant's ledger must instantly reflect the settlement in their home currency to simplify accounting.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      BUYER-CLIENT ||--o{ EDGE-CACHE : "Requests Storefront"
      EDGE-CACHE ||--o{ GEO-LOCATOR : "Determines Region"
      GEO-LOCATOR ||--o{ PRICING-ENGINE : "Requests Localized Price"
      PRICING-ENGINE ||--o{ FX-RATE-PROVIDER : "Fetches Spot Rate"
      BUYER-CLIENT ||--o{ CHECKOUT-GATEWAY : "Initiates Payment"
      CHECKOUT-GATEWAY ||--o{ LOCAL-PAYMENT-METHODS : "Displays Region PMs"
      CHECKOUT-GATEWAY ||--o{ MULTI-CURRENCY-LEDGER : "Records Transaction"
      MULTI-CURRENCY-LEDGER }|--|| CORE-MERCHANT-LEDGER : "Settles Native Currency"
  ```

  ### UI Wireframes & Mobile UX Flow (375px)

  *   **Buyer View (Edge-Localized Storefront - 375px):**
      *   **Action:** A buyer in Berlin visits Priya's OHC store link via Instagram.
      *   **UI:** The edge cache detects the IP in Germany. The storefront instantly displays "Handmade Silver Ring" priced at "€49.99" (not an awkward conversion like €46.32).
      *   **Checkout:** The checkout sheet automatically surfaces "Apple Pay", "Giropay", and "PayPal" (local defaults), with all taxes and duties calculated seamlessly.
  *   **Merchant View (OHC Mobile App - 375px):**
      *   **Action:** Priya receives a push notification: "New Order! You made $54.00 (converted from €49.99)."
      *   **UI:** In her ledger view, she sees the transaction natively in USD. An "Advanced" view shows the original EUR amount and the zero-margin OHC FX conversion rate. She didn't have to configure anything to accept Euros.

  ### Zero Trust & Security

  *   Exchange rate data feeds are signed and validated via SPIFFE workload identities.
  *   The `Pricing-Engine` operates in a secure enclave, ensuring no tenant can manipulate the FX buffer algorithms.

  ### Performance & Offline Targets

  *   **Edge Latency:** Localized price rendering and geo-location must happen in < 50ms via distributed edge nodes (e.g., Cloudflare Workers / Fastly).
  *   **Resilience:** If the live FX feed fails, the Pricing-Engine falls back to the last known 12-hour cached rate with an increased risk buffer margin.

  ### AI Department Coordination

  *   **Finance Agent:** Monitors FX volatility. If a currency drops sharply, the agent automatically triggers the Pricing-Engine to update the localized buffer, ensuring Priya never loses money on an international sale.
  *   **Operations Agent:** Automatically routes the order to the correct international shipping integration, pre-filling customs declarations based on the product catalog.

  ## Implementation Prompt

  **Role:** OHC Implementer Agent
  **Task:** Build the Edge Localization & Multi-Currency Ledger service.
  **Acceptance Criteria:**
  1.  **Zero-Config Activation:** The system must automatically detect the buyer's region (via IP/headers) and present localized pricing and payment methods without the merchant ever toggling a setting.
  2.  **Charming Pricing Algorithm:** The Pricing-Engine must convert the merchant's base price (e.g., $10) using real-time FX rates and apply standard "charm" rounding (e.g., ending in .99, .00, .50) for the local currency.
  3.  **Dual-Ledger Settlement:** When an order completes, write the transaction to a multi-currency ledger in the buyer's currency, and immediately write an atomic settlement entry into the merchant's core ledger in their home currency.
  4.  **Edge Performance:** Design the storefront request flow so that localized pricing is injected at the edge, maintaining an overall page load time under 100ms.
  **Note:** Do not use database-heavy queries for page loads. Leverage edge caching strategies. Do not prescribe specific SQL schemas; focus on the business logic and API contracts for the Pricing-Engine and Ledger interaction.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []