issue_title: "[Architecture] Autonomous Multi-Currency & Localized Pricing Engine"
issue_description: |
  # Autonomous Multi-Currency & Localized Pricing Engine

  ## Problem Statement
  As OneHumanCorp businesses scale beyond their local neighborhoods, they inevitably encounter international customers. Maya (the baker) might get custom order requests from Canada, and Priya (the boutique owner) wants to ship internationally. Currently, OHC lacks a native, zero-configuration system to display prices, process deposits, and handle settlements in the customer's local currency while ensuring the business owner (Priya) gets paid in her home currency without manual conversion math. If Priya has to calculate exchange rates or configure currency zones manually, we've failed our core promise of radical simplicity.

  ## Research Report
  - **Market Gap**:
    - **Shopify**: Requires Shopify Markets, complex configuration of pricing rules, and multi-currency payout setups. Overwhelming for a non-technical user.
    - **Wix/Squarespace**: Limited native support; often relies on third-party apps or static display conversions that don't process at the converted rate.
  - **OHC Opportunity**: Treat currency conversion and localization as a completely invisible infrastructural layer. The system should automatically detect the buyer's locale (Edge/GeoIP), display localized pricing, and handle the Stripe multi-currency processing under the hood.

  ## Design Doc

  ### Architecture & Data Model
  - **Entities**:
    - `CurrencyConfig` (Tenant-level): Home currency of the business.
    - `Price` (Product/Service-level): Stored as integer amounts in the base currency (e.g., USD cents).
    - `ExchangeRateCache`: Redis-backed, periodically updated cache of live exchange rates to ensure UI responsiveness.
  - **Flow**:
    1. Customer visits Priya's storefront.
    2. Edge detects customer's location (e.g., EUR).
    3. Backend calculates display price using base `Price` + `ExchangeRateCache`.
    4. Checkout creates a Stripe PaymentIntent in EUR, configured to settle in Priya's USD account.

  ### AI Department Integration
  - **Finance & Payments (The Accountant)**: Tracks exchange rate fluctuations and automatically notifies Priya in plain language if margin drops significantly due to currency shifts. "Priya, the Euro dropped recently, so your European margins are 3% lower this week. Should I slightly increase European prices?"
  - **Legal & Compliance (The Protector)**: Ensures transparent conversion fee disclosures on the checkout page.

  ### Mobile-First UX
  - **Storefront (375px)**: Seamless auto-detection. A subtle, translucent glass pill at the bottom of the screen indicates "Prices shown in EUR" which the user can tap to override.
  - **Owner Dashboard (375px)**: Priya only ever sees her home currency. Sales from international orders appear as "€100 sale (converted to $108.50)".
  - **Touch Targets**: Standard 44x44px selectors for currency overrides. Native numeric keyboards for manual pricing tweaks if Priya wants fixed localized pricing (e.g., exactly €99).

  ## Implementation Prompt
  Implement the Multi-Currency Pricing Engine. Create the `ExchangeRateCache` mechanism using Redis, update the pricing display endpoints to support optional target currencies, and integrate with Stripe's multi-currency PaymentIntent creation. The UI must include a translucent currency indicator on the storefront and display converted settlements accurately on the owner dashboard without breaking the 375px mobile layout constraint.

  ## Acceptance Criteria
  - Prices auto-convert based on locale header or explicit parameter.
  - Stripe PaymentIntent is created in the target currency.
  - Owner dashboard displays the settled amount in the base currency.
  - E2E Playwright test verifies a cross-currency checkout flow.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, payment]
assignees: []