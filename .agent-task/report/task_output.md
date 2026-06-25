issue_title: "[Architectural Design] Omnichannel Tap-to-Pay Integration & Native Mobile POS Capabilities"
issue_description: |
  ### Problem Statement
  Priya (Boutique Operator) and Fatima (Food Cart Operator) need the ability to take in-person payments efficiently. Currently, OHC handles online digital payments, but lacks native mobile Tap-to-Pay support. For physical retailers and mobile food carts, forcing customers to pay via a web link sent to their phone creates high friction at the point of sale. To be a true operator assistant, OHC must allow the operator to convert their mobile device into an instant POS terminal, syncing in-person transactions with online inventory and analytics seamlessly.

  ### Research Report
  - **Competitor Analysis:**
    - **Shopify POS / Square:** Lead the market in transforming consumer devices into POS systems. They utilize Stripe Terminal SDK or proprietary hardware to enable tap-to-pay natively on iOS and Android.
    - **Stripe Terminal:** Offers Tap-to-Pay on iPhone and Android via native SDKs. This allows merchants to accept contactless payments directly on their smartphone without additional hardware.
  - **Identified Gaps in OHC:**
    - Missing native integrations for Stripe Terminal / Tap-to-Pay.
    - Inventory and cart management are not optimized for a fast-paced in-person checkout flow (e.g., Fatima's food cart needs < 10 second checkouts).
    - Lack of multi-tenant abstraction for physical terminal sessions.

  ### Design Doc

  #### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ POS_SESSION : manages
      POS_SESSION ||--o{ TRANSACTION : processes
      TRANSACTION }|--|| PAYMENT_INTENT : linked_to
      PAYMENT_INTENT ||--|| STRIPE_TERMINAL : executed_via
      TENANT {
          uuid id
          string name
      }
      POS_SESSION {
          uuid id
          uuid tenant_id
          string device_id
          string status
          datetime started_at
      }
      TRANSACTION {
          uuid id
          uuid pos_session_id
          decimal amount
          string currency
          string status
      }
  ```

  #### UI Wireframes & Mobile UX Flow (375px)
  1. **Cart Screen:** A simple, high-contrast, large-touch-target (min 44x44px) list of items in the cart. A prominent, screen-width button "Charge $X.XX" fixed at the bottom.
  2. **Payment Method Screen:** Tap "Charge", slides up a translucent glass bottom sheet offering: "Tap to Pay on Phone", "Send Payment Link", "Cash".
  3. **Tap-to-Pay Active:** If "Tap to Pay" is selected, invokes the native OS Tap-to-Pay UI. The screen dims, waiting for the customer to tap their card or device.
  4. **Success Screen:** A green checkmark with the option to "Text Receipt" or "Email Receipt".

  #### AI Agent Integration Points
  - **Sales & Revenue Assistant:** Automatically tags in-person customers if their card is linked to a previous online purchase.
  - **Operations Assistant:** Adjusts inventory in real-time as physical items are sold.
  - **Decision Assistant:** Compares in-person vs. online sales performance in the daily plain-language summary.

  #### Key Design Decisions and Why
  - **Stripe Terminal SDK via Flutter:** Since OHC uses Flutter for the desktop/mobile app, we will build a bridge to the native Stripe Terminal SDKs for iOS and Android via a Flutter Platform Channel. This provides native performance and access to hardware NFC chips.
  - **Optimistic Inventory Updates:** To ensure fast checkout times in low-connectivity areas (e.g., Fatima's cart), inventory reservations are made optimistically on the device and synced in the background.

  ### Implementation Prompt
  **User-Facing Outcome:** Priya can open the OHC app on her phone, add items to a cart, and select "Tap to Pay". The customer taps their credit card on her phone, the payment is processed, and the inventory is updated automatically.

  **Critical User Journey (CUJ):**
  1. User logs into the OHC mobile app.
  2. User selects the "Point of Sale" tab.
  3. User adds a "T-shirt" to the cart and taps "Charge".
  4. User selects "Tap to Pay". The device prompts for card presentation.
  5. Payment succeeds, and a receipt prompt is shown.
  6. The user's daily summary agent recognizes the sale.

  **Acceptance Criteria:**
  - The Tap-to-Pay flow works seamlessly on a 375px wide viewport.
  - The UI adheres to OHC Premium Token library with translucent materials.
  - Network flakiness does not block the initialization of the cart (though payment authorization will require network).
  - Zero mock data in the UI; items must be fetched from the actual tenant catalog.
  - Fully automated Playwright tests verify the UI flow up to the payment method selection (mocking the hardware native SDK layer in tests).

  ### Priority
  P1

  ### Estimated Scope
  Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
