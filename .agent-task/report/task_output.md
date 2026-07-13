issue_title: "Agentic Multi-Currency & Localized Checkout Architecture"
issue_description: |
  # Research Report: Agentic Multi-Currency & Localized Checkout Architecture

  ## 1. Executive Summary & Market Mapping (Track 1)
  This research explores the challenges non-technical SMB owners face when expanding their sales internationally. Currently, platforms like Shopify require third-party apps, complex multi-currency ledger adjustments, and manual tax code configurations. Our competitors (Wix, Squarespace) offer rudimentary multi-currency displays, but often fail to reconcile at the actual POS checkout accurately or handle dynamic cross-border tax regulations.

  **The Opportunity:** OHC has the opportunity to introduce an "Agentic Multi-Currency & Localized Checkout Architecture," moving the burden from the operator to an AI Finance Agent. This system should dynamically calculate FX conversion, apply local compliance rules (e.g., EU VAT vs. US Sales Tax), and settle payments seamlessly, enabling micro-SMEs to operate globally without needing a finance degree.

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus:** Nora (agency principal), Carlos (field services booking from overseas clients), Priya (boutique with international interest).
  - **The Gap:** OHC currently processes payments primarily via a static setup (e.g., Stripe Checkout Sessions or Stripe Terminal) without an intelligent routing and localized conversion mechanism. If Priya receives an order from the UK, the platform currently lacks the autonomous compliance capability to factor in real-time FX, localization of the payment sheet, and the creation of compliant localized invoicing.
  - **Business Impact:** This friction prevents international growth and results in failed checkouts or legal exposure for the SMB owner.

  ## 3. Architecture Deep Dive (Track 2)
  ### Data Model & Invariants
  - **Multi-Currency Ledger (PostgreSQL):** All items in the database remain priced in the tenant's base currency. A new `PricingLocalization` rule engine applies real-time FX cache modifiers to `Offers` at the point of read.
  - **Localization Edge Cache (Redis):** Caches FX rates and regional tax rates (via external oracle integration like TaxJar or Stripe Tax), refreshed hourly to minimize API calls.
  - **Multi-Tenant Invoice Entity:** The `Invoice` and `PaymentIntent` records must store both the `base_currency_amount` and the `settlement_currency_amount`, along with a snapshotted `fx_rate` applied at checkout.

  ### Architecture Diagram
  ```mermaid
  erDiagram
    TENANT ||--o{ OFFER : owns
    OFFER {
      uuid id
      string base_currency
      int base_price_cents
    }
    REDIS_CACHE ||--o{ FX_RATE : stores
    REDIS_CACHE {
      string key "ohc:fx_rates:{currency_pair}"
      float current_rate
    }
    CHECKOUT_SESSION ||--o{ OFFER : contains
    CHECKOUT_SESSION ||--|{ INVOICE : generates
    INVOICE {
      uuid id
      uuid tenant_id
      int base_currency_amount
      int settlement_currency_amount
      string settlement_currency
      float snapshotted_fx_rate
      float applied_tax_rate
    }
    FINANCE_AGENT }|--|| INVOICE : reconciles
  ```

  ### AI Agent Coordination
  - **Finance Agent ("The Accountant"):** Automatically intercepts a newly generated quote or cart when a foreign IP or shipping address is detected. It dynamically drafts a localized quote displaying both currencies and adjusts tax lines based on local regulations.
  - **Operations Agent ("The Manager"):** Ensures that when an item is sold, the multi-currency invoice is correctly mapped into the daily performance summary in the owner's native currency.

  ## 4. Mobile & Security Integrity (Track 3)
  - **Mobile-First UX (375px):** The checkout flow on a 375px screen must clearly show a localized currency toggle with no horizontal scroll. Payment method inputs must adapt to local norms (e.g., iDEAL for the Netherlands, Klarna for Germany) dynamically via Stripe's Element interface.
  - **Security (Zero Trust):** FX rates and tax modifiers must be cryptographically signed by the backend before being applied to a `CheckoutSession` to prevent client-side tampering of foreign exchange amounts. Multi-tenant boundaries must isolate FX and tax configurations per tenant.

  ## 5. Implementation Prompt
  **Outcome:** Implement the foundational multi-currency backend and edge-cached localization rules engine.
  **Target Persona:** Priya the Boutique Owner
  **Critical User Journey (CUJ):**
  1. A customer in the UK accesses Priya's US-based storefront.
  2. The system detects the region and fetches the localized FX and tax rates from the Redis Edge Cache.
  3. The catalog instantly displays prices in GBP.
  4. The user adds to cart, and the CheckoutSession is generated in GBP.
  5. Priya's dashboard (in USD) receives the order and the Finance Agent provides an updated summary showing the reconciled USD amount minus cross-border fees.

  **Next Actions for Engineering:**
  - Create the `FXCacheService` connecting to our primary Redis instance to store localized exchange rates.
  - Update the `Offer` service layer to accept an optional `target_currency` parameter that applies the cached FX rate dynamically.
  - Extend the existing `Checkout` service to generate `Stripe PaymentIntents` utilizing the localized currency and applying dynamic tax codes.
  - Integrate these flows into a Playwright test.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
