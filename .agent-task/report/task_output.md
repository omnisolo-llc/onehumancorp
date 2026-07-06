issue_title: "[Architecture] Cross-Border Pricing and Payment Engine"
issue_description: |
  # [Architecture] Autonomous Global Multi-Currency & Cross-Border Engine

  ## Problem Statement
  Small business owners frequently lose international sales because presenting prices in a foreign currency creates friction and distrust. Priya (boutique owner) ships globally but struggles to manually calculate exchange rates and update prices for her international customers; she loses sales when Canadian or European buyers see USD prices and abandon their carts. Leo (music tutor) teaches students in the UK and Australia but invoicing them in USD causes confusion and hidden conversion fees for his students. Currently, OHC displays a single base currency for all storefronts and invoices. Business owners lack the time, financial expertise, and technical capability to configure localized pricing, tax routing, and foreign exchange (FX) risk mitigation. They need a system that invisibly handles multi-currency pricing, local payment methods, and automated FX reconciliation without any manual configuration.

  ## Research Report

  We investigated cross-border commerce architectures from leading platforms to understand how they solve localized pricing and FX risk at scale.

  ### Competitive Analysis

  | Platform | Multi-Currency Approach | Key Constraint |
  |---|---|---|
  | Shopify Markets | Powerful, but requires extensive manual setup. FX fees are opaque. | Extremely complex for beginners to configure routing and localized catalogs. |
  | Stripe | Excellent API for localized pricing and presentment. | Developer-centric. Small businesses cannot implement this without a platform layer. |
  | Wix | Basic currency converter widget. | Prices are converted dynamically at checkout, often resulting in "ugly" prices (e.g., €14.82 instead of €14.99). |
  | Squarespace | Very limited. Usually requires third-party plugins. | Brittle integration, poor mobile checkout experience for foreign buyers. |
  | **OHC (Target)** | **Autonomous, zero-config localized presentment and settlement.** | **Must abstract all FX risk and routing complexity from the merchant.** |

  ### Industry Findings
  - **Cosmetic Pricing:** Converting $20 to €18.43 decreases conversion. Best practice is to round to "charming" prices (e.g., €18.99).
  - **Local Payment Methods (LPMs):** Offering iDEAL in the Netherlands or Bancontact in Belgium increases conversion by up to 40% compared to just offering credit cards.
  - **FX Risk:** Merchants want to be paid out in their home currency (e.g., USD) without worrying about daily exchange rate fluctuations.

  ## Design Doc

  ### Key Design Decisions
  1. **Zero-Config Activation:** The engine detects the buyer's IP and browser locale to automatically display the correct currency and LPMs. The merchant does not need to enable "Markets."
  2. **Cosmetic Price Rounding:** The system automatically rounds converted prices to local retail standards (e.g., ending in .99 or .00) while absorbing minor FX variations within a defined threshold.
  3. **Guaranteed Payouts:** The merchant's ledger strictly operates in their home currency. The platform locks in the FX rate at the time of transaction, transferring the FX risk to the payment processor/platform.
  4. **AI-Driven Dispute Resolution:** An AI Finance Agent automatically translates and handles chargebacks or billing inquiries from international customers in their native language.

  ### Architecture Diagram

  ```mermaid
  erDiagram
      MERCHANT ||--o{ STOREFRONT : configures
      MERCHANT {
          string home_currency
          string payout_account
      }
      STOREFRONT ||--o{ SESSION : serves
      SESSION {
          string buyer_ip
          string buyer_locale
          string detected_currency
      }
      SESSION ||--o{ CART : creates
      CART ||--o{ CHECKOUT : transitions_to
      CHECKOUT {
          float localized_total
          string localized_currency
          array available_lpms
      }
      CHECKOUT ||--o| PAYMENT_INTENT : triggers
      PAYMENT_INTENT {
          float amount
          string currency
          float exchange_rate_locked
          float payout_amount_home_currency
      }
      PAYMENT_INTENT ||--|{ LEDGER_ENTRY : writes
  ```

  ```mermaid
  sequenceDiagram
      participant Buyer (Mobile)
      participant Edge Cache (Cloudflare)
      participant OHC Pricing Engine
      participant Payment Gateway (Stripe)
      participant OHC Ledger

      Buyer (Mobile)->>Edge Cache: GET /priyas-boutique (IP: France)
      Edge Cache->>OHC Pricing Engine: Request Pricing Context (EUR)
      OHC Pricing Engine-->>Edge Cache: Return EUR Prices (Cosmetically Rounded)
      Edge Cache-->>Buyer (Mobile): Display Storefront in EUR
      Buyer (Mobile)->>OHC Pricing Engine: Initiate Checkout
      OHC Pricing Engine->>Payment Gateway: Create PaymentIntent (EUR) + Request Local Methods (Cartes Bancaires)
      Payment Gateway-->>Buyer (Mobile): Present Localized Checkout UI
      Buyer (Mobile)->>Payment Gateway: Complete Payment
      Payment Gateway->>OHC Ledger: Webhook: Payment Success (FX Locked)
      OHC Ledger-->>OHC Ledger: Record Payout in Merchant Home Currency (USD)
  ```

  ### Mobile UX Flow (375px First)

  **The "Grandmother Test" Mobile Flow:**
  1. **Storefront View (Buyer):** A buyer in Paris visits Priya's shop on their iPhone. The price tag on a dress elegantly displays "€49.99" instead of "$54.00". A subtle, non-intrusive tooltip says "Showing prices in EUR based on your location. [Tap to change]".
  2. **Checkout View (Buyer):** The checkout drawer slides up. The total is clearly stated in EUR. The payment options prioritize "Cartes Bancaires" and "Apple Pay" at the top, perfectly tailored to a French buyer.
  3. **Dashboard View (Merchant):** Priya opens her OHC app. She sees a push notification: "New sale! €49.99 paid by Chloe in Paris. You will receive $52.10 USD." She doesn't need to do any math.
  4. **Settings (Advanced):** Tucked away under "Settings > International", a simple toggle reads: "Sell Globally: ON. We automatically show local currencies and handle exchange rates."

  ### AI Agent Integration Points
  - **AI Pricing Agent:** Monitors global exchange rates. If a currency drops significantly, it autonomously adjusts the foreign retail price (while respecting cosmetic rounding) to protect the merchant's margin, logging the action for 1-tap approval in the Activity Feed.
  - **AI CS Agent:** If a buyer emails asking about customs duties or shipping delays, the CS agent responds in the buyer's language, pulling context from the cross-border shipping engine.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the Cross-Border Pricing and Payment Engine.
  - **Outcome:** Buyers must see prices natively localized to their region (IP-based) with cosmetic rounding applied automatically. Checkouts must present Local Payment Methods (LPMs) based on the buyer's location. Merchants must see all analytics, sales, and payouts exclusively in their home currency, completely insulated from FX calculations.
  - **CUJ (Critical User Journey):** A buyer in Germany visits a US-based merchant's site. The site loads instantly from the edge cache, displaying prices in EUR. The buyer adds to cart and pays using Giropay. The US merchant receives a single notification showing the sale and their guaranteed USD payout.
  - **Acceptance Criteria:**
    - IP-to-Currency detection functions with < 50ms latency at the edge.
    - Prices are cosmetically rounded (e.g., .99) in the target currency.
    - Multi-tenant data isolation ensures Merchant A's base currency settings never bleed into Merchant B's checkout.
    - The internal ledger accurately records the transaction in both the settlement currency and the merchant's home currency, locking the exchange rate.
    - Offline-capability: The pricing engine should cache current FX rates locally to support offline checkout queuing in intermittent network conditions, syncing securely once online.
    - Payload Targets: Pricing response payloads should be under 50kb to guarantee fast mobile loading times under slow network constraints.
    - Zero Trust & Security: Strict multi-tenant isolation and secure identity are enforced via SPIFFE/SPIRE, ensuring pricing context bounds remain perfectly isolated and immutable.

  ## Priority
  `P1`

  ## Estimated Scope
  `Large`
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
