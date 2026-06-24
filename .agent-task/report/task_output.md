issue_title: "Unified Multi-Currency & Cross-Border Payment Routing Engine"
issue_description: |
  # Research Report: Unified Multi-Currency & Cross-Border Payment Routing Engine

  ## Executive Summary
  This report investigates the architectural gaps in OHC's current cross-border commerce capabilities, identifying critical limitations in multi-currency payment routing. The objective is to design a centralized, AI-driven localization and currency exchange system that empowers small business owners to sell internationally with zero configuration.

  ## 1. Market Mapping & Competitor Discovery
  Competitors like Shopify (Shopify Markets) and Stripe (Global Checkout) provide excellent multi-currency support, dynamically presenting localized pricing, calculating currency-specific fees, and routing transactions to the most cost-effective local payment rail (e.g., SEPA in Europe, iDEAL in Netherlands). However, these platforms require merchants to understand and configure complex tax nexuses, exchange rate risks, and localized banking setups. OHC needs an autonomous system that handles this invisibly.

  ## 2. OHC Gap & Pain Point Identification
  - **Persona Focus:** Priya (boutique owner) has a growing Instagram following in Europe and wants to sell internationally without setting up complex Shopify Markets configurations.
  - **The Gap:** OHC's current payment routing engine (`src/server/integrations/stripe/routing.rs`) has a critical hardcoded assumption: it blindly treats all non-specified currencies as USD. For example, in `optimize_payment_method_with_currency`, if the currency is EUR or GBP, the engine maps `amount` directly to `amount_usd` and calculates Stripe ACH vs Credit Card fees based on US-centric fee structures and US-only rails. ACH is only applicable in the US. This results in failed routing, incorrect fee calculations, and potential transaction failures for cross-border sales.

  ## 3. Deep Dive Architecture Design
  ### Data Model & System Protocol
  - **Multi-Currency Pricing Ledger (PostgreSQL):** Products will have a base price (e.g., USD) with a dynamic, materialized view of localized prices updated via exchange rate streams.
  - **Dynamic Payment Router Service:** The `PaymentRouter` will be extended into an interface-based factory that instantiates the correct localized fee calculator (e.g., SEPACalculator for EUR, BACSCalculator for GBP). It will incorporate real-time conversion rates to correctly estimate margin savings.
  - **CacheConfig (Redis):** Exchange rates and localized product prices will be cached in Redis with a 1-hour TTL, mitigating rate-limiting from external exchange APIs while maintaining sufficient accuracy for checkout rendering.

  ### AI Agent Coordination
  - **Finance Agent ("The Accountant"):** Continuously monitors exchange rate fluctuations. If a currency drops by > 5%, it will proactively suggest adjusting international shipping rates to maintain margins.
  - **Operations Agent ("The Manager"):** Translates international tax requirements automatically during checkout, ensuring compliance without the owner lifting a finger.

  ### Mobile-First Implementation
  - **Mobile UX Flow:** During checkout, the mobile client (375px viewport) reads the customer's locale and fetches the localized price. It displays a seamless single-tap localized payment method (e.g., Bancontact or SEPA) without cluttering the screen with currency dropdowns. Touch targets remain ≥ 44x44px.

  ## 4. Proposed Implementation Prompt
  **Feature Name:** Autonomous Multi-Currency Payment Routing Engine

  **Target Persona:** Priya the Boutique Owner

  **Outcome:** An invisible multi-currency pricing engine that automatically routes international payments to local rails (e.g., SEPA for EUR), calculates correct conversion fees, and prevents erroneous USD ACH fallbacks.

  **Critical User Journey (CUJ):**
  1. Priya's European customer views an online product in EUR on their mobile phone.
  2. The frontend fetches the EUR price from the edge-cached Redis config.
  3. The customer proceeds to checkout. The backend `PaymentRouter` correctly identifies EUR and routes the transaction to SEPA Direct Debit instead of failing over to US ACH.
  4. The Finance Agent calculates the exact exchange fee and logs a multi-currency reconciliation event in the universal ledger.

  **Next Actions for Engineering:**
  - **Step 1:** Refactor `src/server/integrations/stripe/routing.rs` to stop treating unknown currencies as USD. Implement a currency-aware trait/interface that maps EUR to SEPA, GBP to BACS, and calculates fees accurately based on Stripe's localized fee tables.
  - **Step 2:** Integrate an exchange rate caching layer in Redis to convert localized amounts back to the merchant's base currency for accurate margin calculations.
  - **Step 3:** Update `src/server/integrations/stripe/client.rs` to pass correct localized payment method types (e.g., `sepa_debit`) to the Stripe session payload based on the router's output.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
