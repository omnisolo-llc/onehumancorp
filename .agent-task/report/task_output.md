issue_title: "[Architecture] Autonomous Global Multi-Currency Engine"
issue_description: |
  ## Problem Statement
  Small business owners frequently lose international sales because presenting prices in a foreign currency creates friction and distrust. They need a system that invisibly handles multi-currency pricing, local payment methods, and automated FX reconciliation without any manual configuration.

  ## Research Report
  We investigated cross-border commerce architectures from leading platforms to understand how they solve localized pricing and FX risk at scale.
  - **Shopify Markets:** Powerful, but requires extensive manual setup. FX fees are opaque.
  - **Stripe:** Excellent API for localized pricing and presentment, but developer-centric.
  - **Wix:** Basic currency converter widget, often resulting in "ugly" prices (e.g., €14.82).
  - **OHC (Target):** Autonomous, zero-config localized presentment and settlement. Must abstract all FX risk and routing complexity from the merchant.

  ## Design Doc
  **Key Decisions:**
  1.  **Zero-Config Activation:** Detect buyer's IP and browser locale to automatically display the correct currency and LPMs.
  2.  **Cosmetic Price Rounding:** Automatically round converted prices to local retail standards (e.g., ending in .99 or .00).
  3.  **Guaranteed Payouts:** Merchant ledger strictly operates in home currency. FX rate locked at transaction time.
  4.  **AI-Driven Dispute Resolution:** AI Finance Agent automatically translates and handles chargebacks or billing inquiries in the buyer's language.

  **Mobile UX Flow (375px First):**
  - **Storefront View:** Buyer in Paris visits shop. Price elegantly displays "€49.99" instead of "$54.00".
  - **Checkout View:** Total in EUR. Payment options prioritize local methods.
  - **Dashboard View:** Merchant sees push notification: "New sale! €49.99 paid. You will receive $52.10 USD."

  **Architecture Diagram:**
  ```mermaid
  erDiagram
      MERCHANT ||--o{ STOREFRONT : configures
      SESSION ||--o{ CART : creates
      CART ||--o{ CHECKOUT : transitions_to
  ```

  ## Implementation Prompt
  Implement the Cross-Border Pricing and Payment Engine.
  -   **Outcome:** Buyers see prices natively localized to their region with cosmetic rounding. Checkouts present Local Payment Methods based on location. Merchants see analytics, sales, and payouts in home currency.
  -   **CUJ:** Buyer in Germany visits US merchant's site, sees prices in EUR, pays via Giropay. US merchant receives notification showing USD payout.
  -   **Acceptance Criteria:**
      -   IP-to-Currency detection < 50ms latency.
      -   Cosmetic rounding (e.g., .99) in target currency.
      -   Strict multi-tenant isolation.
      -   Ledger records transaction in both settlement and home currency, locking exchange rate.
      -   Offline-capability for caching FX rates.
      -   Payloads < 50kb.
      -   Zero Trust security via SPIFFE/SPIRE.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
