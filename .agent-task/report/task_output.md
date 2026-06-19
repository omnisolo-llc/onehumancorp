issue_title: "Implement Hardware-Free Tap-to-Pay Integration for In-Person Operations"
issue_description: |
  **Problem Statement**
  Priya (boutique operator) and Fatima (food cart owner) handle a significant volume of in-person, on-the-spot transactions. While OHC handles online checkouts well, forcing in-person customers to scan a QR code, open a payment link, or use external physical POS hardware breaks the frictionless experience they expect. They need the ability to securely tap a customer's credit card or phone directly against their own smartphone (Tap-to-Pay on iPhone/Android) to complete an order instantly.

  **Research Report**
  - **Market Context**: Square dominated the physical POS market for years with specialized hardware. Recently, Stripe Terminal introduced Tap-to-Pay on mobile devices without external readers. Shopify POS and Wix POS leverage this heavily to unify online and offline inventory and payments.
  - **The OHC Opportunity**: Integrating Tap-to-Pay directly into the OHC Flutter/PWA mobile app allows operators like Priya and Fatima to use their existing phones as checkout terminals. This keeps all transaction data, inventory sync, and revenue tracking unified within OHC, bypassing external hardware costs.
  - **Competitor Gaps**:
    - *Shopify POS*: Powerful but often requires complex synchronization with online store settings and monthly tier upgrades.
    - *Square*: Fragmented ecosystem if the user also wants advanced AI-driven customer follow-ups and omnichannel CRM capabilities like OHC offers.

  **Design Doc**
  - **Architecture**:
    - Mobile App (Flutter/PWA): Integrate `stripe_terminal` SDK. The app will request location permissions (required by Stripe) and negotiate a secure Tap-to-Pay session with the device's NFC chip.
    - OHC API (Rust/Axum): New endpoints under `src/server/services/payments` to generate ConnectionTokens for the Terminal SDK and handle PaymentIntents specifically flagged for card-present transactions.
    - Data Model: Extensions to the Payment entity to support offline/card-present metadata, device tracking (for security/auditing), and location data.
  - **Mobile UX Flow (375px)**:
    1. Operator creates an order or selects a cart on the mobile app.
    2. Taps "Charge (Tap to Pay)".
    3. App invokes native iOS/Android Tap-to-Pay UI (a secure system overlay).
    4. Customer taps their card or phone.
    5. Native UI dismisses; OHC app displays success screen and asks if a digital receipt should be sent via SMS/Email (handled by Customer Success Agent).
  - **AI Agent Integration**:
    - *Finance Assistant*: Automatically reconciles card-present transactions with the daily ledger, marking them cleared instantly.
    - *Customer Assistant*: Captures digital receipt preferences and links the transaction to a new or existing customer profile based on email/phone entry.

  **Implementation Prompt**
  **Feature Name**: Native Tap-to-Pay Checkout
  **Target Personas**: Priya (Boutique) & Fatima (Food Cart)
  **Outcome**: Operators can process in-person payments directly on their smartphones without external card readers, seamlessly integrating with OHC's unified inventory and ledger.

  **Next Actions**:
  1. Add `stripe_terminal` dependencies and required mobile permissions (Location, NFC/Bluetooth) to the mobile app.
  2. Implement the `POST /api/v1/payments/terminal/connection_token` endpoint in the Rust backend to securely issue short-lived Stripe tokens.
  3. Build the Tap-to-Pay mobile checkout flow, ensuring a 375px-optimized experience for selecting an order and launching the native payment overlay.
  4. Ensure backend webhooks and Finance Assistant properly categorize these transactions as `card_present` and update daily revenue summaries.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
