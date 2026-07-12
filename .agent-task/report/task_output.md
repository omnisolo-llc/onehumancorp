issue_title: "Research: Automated Multi-Tenant Localized Pricing & Currency Conversion"
issue_description: |
  # Research Report: Automated Multi-Tenant Localized Pricing & Currency Conversion Architecture

  ## 1. Problem Statement
  As OneHumanCorp targets a global SMB audience, business owners like Maya (baker) or Leo (music tutor) face difficulties when serving international customers. Current legacy platforms (Shopify, Wix) often require expensive third-party apps or high-tier plans to display localized pricing and handle multi-currency checkouts. Without automated localized pricing, international customers experience high checkout abandonment due to currency confusion and unexpected conversion fees.

  ## 2. Research & Market Landscape
  - **Competitor Gaps**:
    - *Shopify*: Multi-currency requires Shopify Markets (complex setup) or third-party apps.
    - *Wix / Squarespace*: Limited native support for dynamic currency conversion based on geolocation.
    - *Stripe*: Provides excellent APIs for localized pricing, but requires technical integration to utilize seamlessly across a storefront.
  - **OHC Opportunity**: Implement an invisible, edge-cached, dynamic currency conversion system that leverages Stripe's localized pricing APIs. This will automatically display correct currencies to users based on their GeoIP without requiring the business owner to configure complex exchange rates or install plugins.

  ## 3. Design Document (Architecture & Data Model)
  ### Data Model (PostgreSQL)
  - `TenantConfig`: Extend with `base_currency` (e.g., USD, EUR).
  - `Product` / `Service`: Prices are stored in the tenant's `base_currency` in the lowest denomination (e.g., cents).

  ### Component Architecture
  1. **Edge/Gateway Layer**:
     - Intercept incoming requests to identify the user's country code via GeoIP (e.g., `Cloudflare-IPCountry` header).
  2. **Pricing Engine & Caching**:
     - Maintain an exchange rate cache (Redis) refreshed periodically (e.g., daily) from a reliable provider (or Stripe).
     - Dynamically convert product prices from the tenant's `base_currency` to the user's localized currency if supported, rounding to logical localized price points (e.g., $19.99 instead of $19.83).
  3. **Stripe Integration**:
     - When creating Stripe Checkout Sessions or Payment Intents, pass the localized currency and amount to ensure the user is billed in their local currency, avoiding surprise bank fees.
  4. **Operations Agent**:
     - Monitors international sales and notifies the business owner of trends (e.g., "You had 10 orders from Canada this week. Consider adjusting your Canadian shipping rates.")

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User
      participant Edge as Edge/Gateway (GeoIP)
      participant API
      participant Cache as Redis (Exchange Rates)
      participant DB as Postgres (Tenant/Product)
      participant Stripe

      User->>Edge: Requests Storefront
      Edge->>API: Forwards request with GeoIP Header (e.g. Country: CA)
      API->>DB: Fetches Product Price in Base Currency (USD)
      API->>Cache: Fetches Exchange Rate (USD to CAD)
      API->>User: Returns Localized Pricing (CAD)

      User->>API: Initiates Checkout
      API->>Stripe: Creates Checkout Session in CAD
      Stripe->>User: Renders Stripe Checkout in CAD
  ```

  ### Mobile UX Flow (375px)
  - **Customer View**: The storefront automatically displays prices in the customer's local currency (indicated by a subtle currency flag or symbol).
  - **Owner View**: The owner sets the price once in their base currency. All reports and payouts are shown in their base currency, hiding the complexity of exchange rates.

  ## 4. Implementation Prompt
  **Feature Name**: Edge-Native Localized Pricing & Checkout
  **Target Persona**: Maya the Baker (shipping specialty items internationally)
  **Outcome**: Maya's storefront automatically detects the customer's location, converts prices to their local currency using a cached exchange rate, and processes the payment in the local currency via Stripe, all without Maya configuring anything.

  **Next Actions for Engineering**:
  1. Integrate GeoIP detection at the gateway/router layer.
  2. Implement an exchange rate caching service using Redis, fetching daily rates.
  3. Update the Product/Catalog APIs to dynamically apply localized pricing based on the detected GeoIP and cached rates.
  4. Ensure Stripe Checkout Sessions are instantiated with the localized currency amount.
  5. Add an E2E Playwright test simulating an international user browsing and checking out.
  6. Address top confusing items discovered in repository.

  **Top 5 Confusing Things in Repository to Fix Later**:
  1. The `.bazelversion` is empty or not matching a standard format, which could cause inconsistent builds across environments.
  2. The `setupTests.ts` is in the root directory rather than grouped closely with the frontend code.
  3. The `mcp_memory_recording.json` sits in the repo root without clear documentation on its role outside agent memory.
  4. Redundant docker compose configurations (`docker-compose.override.yml` is essentially empty but kept around).
  5. Next.js vs Tauri v2 confusion in `docs/business/market_research/ux_analysis_onboarding.md` implies some legacy code needs pruning (`src/ui/next/`).

  **Priority**: P2
  **Estimated Scope**: Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
