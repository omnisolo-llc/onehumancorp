issue_title: "Research: Tap-to-Pay POS Integration for SMB Terminal Mobility"
issue_description: |
  # Research: Tap-to-Pay POS Integration for SMB Terminal Mobility

  ## Problem Statement
  SMB operators like Priya (boutique operator) and Fatima (food cart operator) lose sales when relying on clunky, stationary legacy card readers or QR-code only systems. They need the ability to instantly accept contactless payments (Apple Pay, Google Pay, physical cards) directly on their primary mobile device (iPhone/Android) without needing extra hardware dongles. The lack of a native "Tap-to-Pay on iPhone/Android" feature forces them to use external POS systems, breaking the unified OHC workflow, fragmenting their inventory/sales data, and increasing operational overhead.

  ## Research Report
  - **Competitor Analysis:**
    - *Shopify:* Offers "Tap to Pay on iPhone" via the Shopify POS app. This has significantly reduced the barrier to entry for in-person sellers.
    - *Square:* The pioneer here; recently integrated Apple's Tap to Pay, meaning sellers no longer even need the iconic Square magstripe/chip reader dongle.
    - *Stripe:* Provides Terminal SDKs that support Tap to Pay on compatible iOS and Android devices. This is the gold standard for developer integration.
  - **Technical Feasibility:** Stripe Terminal SDKs are available for React Native, iOS (Swift), and Android (Kotlin). Given OHC uses Flutter, we can leverage community/official plugins wrapping these SDKs (e.g., `stripe_terminal` for Flutter) to enable this within the OHC unified mobile shell.
  - **Business Impact:** High. Eliminating hardware costs and setup time for in-person payments directly impacts activation rates and Gross Merchandise Volume (GMV) processed through OHC. It strongly aligns with the "run from phone" promise.

  ## Design Doc
  - **Architecture:**
    - Mobile Client (Flutter): Integrates Stripe Terminal SDK. Requests temporary connection tokens from OHC Backend.
    - OHC Backend (Go): Exposes `/api/v1/terminal/connection_token` endpoint. Uses Stripe Go library to generate tokens scoped to the tenant's connected Stripe account.
    - Payment Flow: Mobile client initializes reader (Tap to Pay mode) -> Creates PaymentIntent via OHC Backend -> Collects payment method via device NFC -> Confirms PaymentIntent via OHC Backend -> Backend updates ledger and inventory.
  - **Mobile UX Flow (375px):**
    1.  Owner taps "Charge $XX.XX" on the order/cart screen.
    2.  Bottom sheet slides up: "Select Payment Method". Options: "Tap to Pay on Phone", "Send Payment Link", "Cash".
    3.  Owner selects "Tap to Pay". Native OS overlay (Apple/Google) appears: "Hold card near top of phone".
    4.  Customer taps card.
    5.  Success animation -> Receipt options (SMS/Email) -> Return to order feed.
  - **AI Agent Integration:**
    - *Finance & Decision Assistant:* Logs the transaction instantly. If a pattern of in-person sales emerges, it might suggest optimizing the physical storefront or creating local promotional campaigns.
    - *Customer Relationship Assistant:* If the customer is recognized (e.g., via digital receipt link interaction), adds the transaction to their profile for future personalized offers.

  ## Implementation Prompt
  **User-Facing Outcome:** Enable physical business owners (like Priya and Fatima) to accept in-person contactless payments directly using their smartphone's NFC chip, without requiring external card reader hardware.
  **CUJ:**
  1. Owner builds a cart or creates a quick charge amount in the OHC mobile app.
  2. Owner initiates checkout and selects "Tap to Pay on Phone".
  3. The app connects to the terminal securely and displays the native OS Tap-to-Pay interface.
  4. Customer taps their card or device.
  5. Payment succeeds, order is marked paid, and inventory is updated.
  **Acceptance Criteria:**
  - Flutter app integrates the necessary Stripe Terminal libraries.
  - Backend provides secure endpoints for Terminal connection tokens and PaymentIntent management.
  - The UI gracefully handles hardware capability checks (e.g., falling back to manual entry or payment links if the device lacks NFC/Tap-to-Pay support).
  - Transactions are correctly attributed to the tenant and visible in the OHC feed.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
