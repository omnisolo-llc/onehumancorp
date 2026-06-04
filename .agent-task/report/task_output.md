issue_title: "[Architecture] Zero-Config Universal Tap-to-Pay POS Engine via Stripe Terminal"
issue_description: |
  # Zero-Config Universal Tap-to-Pay POS Engine

  ## Problem Statement
  For business owners like **Priya (The Boutique Owner)** and **Fatima (The Food Cart Operator)**, accepting in-person payments currently requires expensive external dongles (Square, old Stripe readers) or relying on separate, disjointed point-of-sale applications. When Priya uses a separate POS app for her in-store sales, her online inventory falls out of sync, leading to double-selling. She needs the ability to accept contactless credit card payments seamlessly on her existing mobile phone (iPhone or Android) with zero extra hardware. The solution must be natively embedded into the OneHumanCorp (OHC) app to ensure inventory, financial reporting, and customer identity remain perfectly synchronized.

  ## Research Report
  **Competitor Analysis:**
  - **Shopify POS:** Excellent system, but requires a separate app download and often encourages the purchase of external hardware for optimal use.
  - **Square:** The dominant player for physical retail, but their e-commerce tools are historically weaker. Setup can feel fragmented between the hardware POS and the online store.
  - **Wix/Squarespace:** Lack robust, natively integrated mobile-first POS systems that turn the merchant's personal phone into a terminal without dongles.

  **Opportunity for OHC:** By integrating Stripe Terminal's SDK directly into the OHC mobile application, we can enable "Tap to Pay on iPhone" and "Tap to Pay on Android." This allows the OHC app to function as a complete, hardware-free POS system. This "zero-config" approach perfectly aligns with OHC's mission of radical simplicity.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Device
          App[OHC Mobile App 375px] --> TerminalSDK[Stripe Terminal Mobile SDK];
          App --> LocalDB[(Local Cache / Sync Engine)];
          TerminalSDK -->|NFC Read| Card[Customer Credit Card / Apple Pay];
      end

      App -- Create Connection Token --> Gateway[OHC API Gateway];
      App -- Process Payment Intent --> Gateway;
      Gateway --> Services[OHC Payment & Inventory Services];
      Services --> MainDB[(Cloud Postgres)];
      Services --> StripeCloud[Stripe API];
      Services --> Agents[AI Operations & Finance Agents];
  ```

  ### UX Flow & Mobile-First Design (375px)
  1. **Checkout UI:** From the OHC app dashboard, the user taps a prominent "Charge" FAB.
  2. **Amount/Item Entry:** A clean, numeric keypad (native mobile keyboard) appears, or the user can select an item from their synchronized catalog to build a cart.
  3. **Payment Method:** The user selects "Tap to Pay". The screen transitions using a smooth Glassmorphism overlay (`backdrop-filter: blur(20px)`), dimming the background.
  4. **NFC Interaction:** The native OS "Tap to Pay" interface takes over, instructing the customer to hold their card near the top of the phone.
  5. **Confirmation:** A large, highly visible green checkmark with a haptic success vibration confirms the payment. The user is instantly returned to the OHC app, showing a receipt option (SMS/Email).

  ### AI Agent Integration
  - **The Operations Agent:** Instantly decrements the purchased items from the unified inventory mesh. If stock falls below a threshold, it queues a restocking notification for the user.
  - **The Finance Agent:** Reconciles the local POS transaction into the unified double-entry ledger, ensuring the funds are accounted for in the daily payout batch.
  - **The Customer Success Agent:** If the user opted to send a receipt via SMS (which captures their phone number), the agent uses the cross-channel identity graph to link this offline purchase to any existing online profile, enabling future marketing.

  ## Implementation Prompt
  **Goal:** Implement the backend connection token service and outline the frontend SDK integration for the Tap-to-Pay POS.
  **Acceptance Criteria:**
  - Create a new backend service endpoint (e.g., `POST /api/v1/payments/terminal/token`) that securely generates and returns a Stripe Terminal Connection Token, scoped to the current tenant.
  - Ensure the endpoint validates tenant permissions and logs the request via OpenTelemetry.
  - Update the `Order` or `Payment` data models to support a `payment_source` enum containing `STRIPE_TERMINAL_TAP`.
  - Provide 100% unit test coverage for the token generation logic.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
