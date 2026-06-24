issue_title: "[Research] Native Tap-to-Pay Integration for Physical Retail"
issue_description: |
  # Native Tap-to-Pay Integration for Physical Retail

  ## Problem Statement
  Small business owners like Priya (Boutique Operator) need a unified way to process in-person payments without managing separate point-of-sale systems or external card readers. Currently, OHC focuses heavily on digital payments and invoices, but lacks a native, mobile-first Tap-to-Pay solution for physical retail scenarios, forcing users to leave the OHC ecosystem or reconcile manual payments.

  ## Research Report
  - **Competitive Analysis**: Systems like Shopify POS, Square, and Stripe Terminal have popularized turning smartphones into payment terminals. Shopify's "Tap to Pay on iPhone" and Android equivalent allow merchants to accept contactless payments directly on their mobile devices without additional hardware.
  - **Market Need**: A significant portion of our target personas (Priya, Fatima, Carlos) operate in physical spaces. A seamless Tap-to-Pay experience is critical for reducing friction at checkout and keeping all transaction data unified within the OHC platform.
  - **Technical Feasibility**: Stripe provides SDKs for both iOS and Android to enable Tap to Pay. Flutter plugins exist to wrap these SDKs, allowing integration into our existing mobile-first architecture.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
    participant OHC Mobile App
    participant Stripe Terminal SDK
    participant OHC Backend
    participant Stripe API

    OHC Mobile App->>OHC Backend: Request Connection Token
    OHC Backend->>Stripe API: Create Connection Token
    Stripe API-->>OHC Backend: Return Connection Token
    OHC Backend-->>OHC Mobile App: Return Connection Token
    OHC Mobile App->>Stripe Terminal SDK: Initialize & Connect Reader (Local Mobile)
    OHC Mobile App->>OHC Backend: Create PaymentIntent (Amount)
    OHC Backend->>Stripe API: Create PaymentIntent
    Stripe API-->>OHC Backend: Return Client Secret
    OHC Backend-->>OHC Mobile App: Return Client Secret
    OHC Mobile App->>Stripe Terminal SDK: Collect Payment Method (Tap)
    Stripe Terminal SDK->>OHC Mobile App: Payment Method Collected
    OHC Mobile App->>Stripe Terminal SDK: Process Payment
    Stripe Terminal SDK->>Stripe API: Confirm Payment
    Stripe API-->>Stripe Terminal SDK: Payment Success
    Stripe Terminal SDK-->>OHC Mobile App: Payment Success
    OHC Mobile App->>OHC Backend: Verify & Record Transaction
  ```

  ### Mobile UX Flow
  1. **Initiate Checkout**: Owner adds items to the cart or enters a custom amount on the OHC mobile app.
  2. **Select Tap-to-Pay**: Owner taps "Tap to Pay" as the payment method.
  3. **Present Device**: The native OS Tap-to-Pay sheet appears. The owner presents the phone to the customer.
  4. **Tap Card/Device**: Customer taps their contactless card or mobile wallet on the owner's phone.
  5. **Processing & Success**: The screen shows processing, then a success checkmark. The OHC app records the sale and updates inventory/analytics.

  ### AI Agent Integration Points
  - **Finance & Decision Assistant**: Automatically logs the transaction, updates daily revenue summaries, and reconciles the payment against the corresponding order.
  - **Customer & Relationship Assistant**: If the customer is recognized (e.g., via digital receipt opt-in), updates their purchase history and triggers any relevant follow-up actions.

  ## Implementation Prompt
  Implement a new `TapToPayService` in the Flutter frontend and corresponding backend endpoints to support Stripe Terminal Tap to Pay.

  **Acceptance Criteria**:
  - Implement a mockable `TapToPayService` interface in the frontend for local development and testing.
  - Create backend endpoints for generating Stripe Terminal Connection Tokens and creating PaymentIntents for Terminal.
  - Design a 375px-optimized checkout flow UI that includes a "Tap to Pay" option.
  - Ensure the UI handles connection states, reading states, success, and error states gracefully with translucent glass styling.
  - Provide a robust mock implementation of the Stripe Terminal SDK for E2E testing without actual hardware.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
