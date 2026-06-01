issue_title: "[Architecture] Implement In-Person POS Payments via Tap-to-Pay (Stripe Terminal SDK)"
issue_description: |
  ## Problem Statement
  Priya, a 35-year-old boutique owner, sells clothing both in-store and online. Currently, she struggles to maintain synchronized inventory and unified analytics because her in-person sales (Point of Sale) are disconnected from her online storefront. She needs a seamless way to accept in-person tap-to-pay payments directly on her iPhone, immediately syncing with her OHC inventory and financial tracking without requiring specialized, clunky hardware terminals.

  ## Research Report
  - **Findings**: The current OHC ecosystem handles online payments efficiently (via Stripe Checkout and Intents), but lacks native, deeply integrated offline POS capabilities that leverage a merchant's existing mobile device.
  - **Competitive Analysis**: Shopify offers Shopify POS, which is robust but requires an additional subscription tier or proprietary hardware. Wix and Squarespace have limited in-person capabilities, often relying on third-party integrations like Square.
  - **Strategic Opportunity**: By natively integrating the Stripe Terminal SDK (Tap to Pay on iPhone/Android) directly into the OHC Flutter mobile app, OHC can offer a zero-hardware, zero-configuration POS system. This reinforces the OHC promise: a full business stack manageable from a phone.

  ## Design Doc
  ### Architecture
  ```mermaid
  graph TD
      A[Flutter Mobile App] -->|Stripe Terminal SDK| B(Tap-to-Pay Interface)
      A -->|gRPC| C[OHC API Layer]
      B -->|Payment Intent| C
      C -->|Stripe API| D{Stripe Backend}
      C --> E[PostgreSQL - Ledger/Inventory]
      C --> F[Redis - Redlock for Inventory]
      E --> G[Finance & Payments Agent]
  ```
  ### Mobile UX Flow (375px first)
  1. Priya opens the OHC app and selects "New In-Person Sale".
  2. She adds products to the cart (inventory checked in real-time).
  3. She taps "Charge $XX.XX".
  4. The native Stripe Terminal SDK UI pops up, prompting the customer to tap their card/phone.
  5. Upon success, a "Payment Approved" glassmorphism success screen appears.
  6. The Finance agent updates daily analytics; the Operations agent deducts inventory.
  ### AI Agent Integration Points
  - **Finance & Payments ("The Accountant")**: Reconciles the physical payment with online sales for a unified daily report.
  - **Operations ("The Manager")**: Instantly deducts the purchased item from the shared physical/digital inventory.
  - **Customer Success ("The Ambassador")**: If the customer provides an email or is recognized (via digital receipt), sends a branded "Thank you for visiting us in-store!" email.

  ## Implementation Prompt
  Implement the full-stack infrastructure for In-Person POS Payments using Stripe Terminal (Tap to Pay).
  - **User Journey (CUJ)**: Priya launches the OHC mobile app, creates a new cart from her inventory, initiates a Tap-to-Pay session on her phone, the customer pays, and Priya's dashboard immediately reflects the new revenue and updated inventory.
  - **Acceptance Criteria**:
    1. Integrate Stripe Terminal SDK into the Flutter app.
    2. Create backend endpoints to generate Terminal Connection Tokens and handle PaymentIntents for physical terminals.
    3. Ensure row-level tenant isolation is maintained during the transaction.
    4. Ensure the transaction seamlessly updates the unified inventory system, utilizing Redis locks to prevent race conditions.
    5. The UI must be fully functional on a 375px screen and adhere to the OHC Premium Token library (Glassmorphism).

  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
