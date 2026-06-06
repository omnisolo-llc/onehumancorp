issue_title: "[Research] Mobile-First Tap-to-Pay POS Architecture Integration"
issue_description: |
  # Research Report: Mobile-First Tap-to-Pay POS Architecture Integration

  ## Problem Statement
  Priya, our boutique owner persona, sells clothing in-store and online. She needs her OHC storefront synchronized with her in-store inventory and requires in-person POS payments via a phone tap-to-pay or card reader. Legacy platforms either completely ignore in-person sales or require separate, clunky POS hardware that does not integrate seamlessly with the online store, causing inventory mismatches.

  ## Research Report & Gap Analysis
  - **Shopify**: Excellent online and POS integration, but requires dedicated Shopify POS hardware or their specific app, which can be complex to setup for a pure "phone-only" experience.
  - **Wix/Squarespace**: Limited native POS integration; often relies on clunky third-party syncing or external card readers that break the unified experience.
  - **Square**: Strong POS, but building a beautiful, unified online storefront is less customizable compared to OHC's vision.
  - **OHC Gap**: OHC currently lacks a seamless, unified architecture for processing in-person payments directly from a 375px mobile device (Tap-to-Pay) while maintaining strict multi-tenant isolation and instant inventory synchronization with the online catalog.

  ## Design Doc
  ### Mobile-First UX Flow
  1. Priya opens the OHC mobile app (Operations Feed) and taps "New In-Store Sale".
  2. A streamlined, 375px-optimized product grid appears. She taps items to add to the cart.
  3. She taps "Charge $45.00".
  4. The screen transitions to a "Tap to Pay" interface using Stripe Terminal SDK.
  5. Customer taps their card/phone.
  6. Success screen with an option to email/text receipt. Inventory is instantly deducted globally.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant App as Mobile App (Flutter/PWA)
      participant OHC as OHC Backend (Go/Rust)
      participant DB as Postgres (Tenant Isolated)
      participant Stripe as Stripe Terminal API

      App->>OHC: Initialize Tap-to-Pay Session (Tenant ID)
      OHC->>Stripe: Create ConnectionToken
      Stripe-->>OHC: Token
      OHC-->>App: Token
      App->>Stripe: Connect Reader (Local/NFC)
      App->>OHC: Create PaymentIntent (Amount, Cart Items)
      OHC->>DB: Hold Inventory (Transaction)
      OHC->>Stripe: Create PaymentIntent
      Stripe-->>OHC: Client Secret
      OHC-->>App: Client Secret
      App->>Stripe: Process Payment (Tap)
      Stripe-->>App: Payment Success
      App->>OHC: Confirm Payment
      OHC->>DB: Commit Inventory Deduction, Record Sale
      OHC-->>App: Receipt Generated
  ```

  ### Implementation Prompt
  **Implementer Instructions**:
  Design and implement the necessary data structures and backend endpoints to support a mobile-first, in-person Tap-to-Pay experience using Stripe Terminal.
  1. Define the Entity-Relationship models for `TerminalSession` and `InPersonOrder` ensuring strict multi-tenant isolation.
  2. Implement the API endpoints for initializing a Stripe Terminal connection token and creating an in-person `PaymentIntent`.
  3. Ensure the `Finance & Payments` AI department can observe these transactions to generate correct unified financial reports.
  4. Design the 375px mobile UX using OHC Premium Tokens (Glassmorphism) for the Tap-to-Pay interface.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
