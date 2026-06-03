issue_title: "[Architecture] Autonomous Global Multi-Currency & Cross-Border Engine"
issue_description: |
  ## Problem Statement
  Small business owners frequently lose international sales because presenting prices in a foreign currency creates friction and distrust. They need a system that invisibly handles multi-currency pricing, local payment methods, and automated FX reconciliation without manual configuration.

  ## Research Report
  - Competitor Analysis: Shopify Markets is powerful but requires manual setup; Stripe is developer-centric; Wix relies on third-party apps and provides ugly dynamic converted prices.
  - OHC Differentiator: Autonomous, zero-config localized presentment and settlement. Abstract all FX risk and routing complexity from the merchant. Cosmetic pricing and local payment methods must be handled instantly.

  ## Design Doc
  - **Zero-Config Activation**: Detect buyer IP and browser locale to automatically display the correct currency and LPMs.
  - **Cosmetic Price Rounding**: Automatically round converted prices to local retail standards (e.g., .99).
  - **Guaranteed Payouts**: The merchant's ledger strictly operates in their home currency.
  - **Mobile UX Flow (375px First)**: Storefront and Checkout natively display localized pricing. Dashboard shows only the home currency payout.

  ## Implementation Prompt
  Implement the Cross-Border Pricing and Payment Engine. Ensure that buyers see prices natively localized to their region with cosmetic rounding applied automatically. Checkouts must present Local Payment Methods based on the buyer's location. Merchants must see all analytics and payouts in their home currency.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
