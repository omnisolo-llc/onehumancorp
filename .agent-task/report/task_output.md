---
issue_title: "[Architecture] Zero-Config Global Multi-Currency Pricing Engine"
issue_description: |
  # Zero-Config Global Multi-Currency Pricing Engine

  ## Problem Statement
  Small business owners (like Priya the boutique owner or Leo the music tutor) increasingly reach international customers but struggle immensely with cross-border sales. Currently, showing localized pricing, managing exchange rate fluctuations, handling cross-border transaction fees, and ensuring local tax compliance requires configuring complex multi-currency apps or manual price lists. This causes severe "Financial Fog" and cart abandonment when international buyers see foreign currencies or unexpected fees at checkout. Business owners need a zero-config, invisible multi-currency engine that automatically localizes storefront pricing, handles real-time conversion, and simplifies payouts in their native currency without any manual setup.

  ## Research Report
  *   **Shopify:** Offers Shopify Markets, which is powerful but requires significant configuration. Merchants must manually enable markets, configure price rounding rules, and manage exchange rate risk. It is overwhelming for a first-time smartphone user.
  *   **Wix:** Multi-currency is often a display-only feature (customers see local currency but check out in the merchant's base currency, causing confusion) or requires third-party apps for full localized checkout.
  *   **Squarespace / GoDaddy:** Very limited native multi-currency support, usually forcing a single base currency for checkout.
  *   **OneHumanCorp (OHC) Differentiation - "Invisible Global Commerce":** OHC eliminates the concept of "Markets" configuration. The platform automatically detects the buyer's locale via Edge Caching and Zero Trust routing, presenting localized, rounded prices. The AI Finance Agent handles exchange rate buffering and transparently quotes the final payout to the merchant in their base currency, making global sales feel exactly like local sales.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      BUYER_DEVICE ||--o{ EDGE_ROUTER : "Requests localized storefront"
      EDGE_ROUTER }|--|| CURRENCY_CACHE : "Fetches exchange rates & rules"

      EDGE_ROUTER ||--o{ AI_FINANCE_AGENT : "Requests dynamic pricing quote"

      AI_FINANCE_AGENT {
          string base_currency "Merchant's native currency"
          float exchange_buffer "Risk buffer"
      }

      AI_FINANCE_AGENT ||--o{ LEDGER_CORE : "Locks final payout amount"
      AI_FINANCE_AGENT ||--o{ PAYMENT_GATEWAY : "Initiates localized capture"

      LEDGER_CORE {
          string tenant_id "Multi-tenant isolation"
      }
  ```

  ### UI Wireframes & Mobile UX Flow (375px first)
  *   **Merchant View (Global Sales Toggle):**
      *   No complex "Markets" list.
      *   **Screen:** A clean macOS-style Translucent Glass card under "Store Settings".
      *   **Toggle:** "Sell Internationally (Auto-Convert Pricing)" [ON/OFF].
      *   **Description:** "We'll automatically show local prices to international buyers and pay you in USD."
      *   **Interaction:** 1-tap enable. No other configuration required.
  *   **Buyer View (Edge Localization):**
      *   **Screen:** The 375px mobile storefront.
      *   **Behavior:** Buyer in the UK visits Priya's US-based store. The Edge Router detects the IP/locale.
      *   **Display:** Prices are seamlessly displayed as "£45" (cleanly rounded, not £44.32) instead of "$50".
      *   **Checkout:** Checkout proceeds in GBP.

  ### AI Agent Integration Points
  *   **AI Finance Department:** Monitors real-time exchange rates. When an international buyer views a product, the AI instantly calculates a localized price that protects the merchant's base margin, applying smart psychological rounding (e.g., ending in .99 or .00).
  *   **AI Operations Agent:** Automatically flags any unusual cross-border shipping costs and alerts the merchant if an order's fulfillment cost outweighs the localized profit margin.

  ### Key Design Decisions
  *   **Display vs. Checkout Parity:** Buyers MUST check out in the exact currency they see on the storefront. No bait-and-switch at the final step.
  *   **Zero Merchant Risk:** The merchant's ledger only ever shows their base currency. All exchange rate volatility is buffered and managed invisibly by the platform/AI.
  *   **Edge-First Pricing:** Localized pricing must be calculated and cached at the edge to maintain strict performance latency targets (<50ms) for the storefront.

  ## Implementation Prompt
  Develop the core pricing engine and backend ledger support to enable zero-config multi-currency sales. The solution should include an Edge-aware pricing service that intercepts product catalog requests and dynamically injects localized pricing based on the buyer's locale. It must integrate with the AI Finance Agent to calculate risk-adjusted, psychologically rounded prices. The checkout and ledger components must be updated to capture the foreign currency while recording the merchant's guaranteed payout in their base currency. Ensure strict multi-tenant isolation within the Ledger Core. Create the corresponding "1-tap" UI toggle for the merchant dashboard using the designated glassmorphism design tokens.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
---
