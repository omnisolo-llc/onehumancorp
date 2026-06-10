issue_title: "Implement In-Person Tap-to-Pay Integration for POS using Stripe Terminal"
issue_description: |
  # Mission Queue Protocol: Implement In-Person Tap-to-Pay Integration

  ## Title
  Implement In-Person Tap-to-Pay Integration for POS using Stripe Terminal

  ## Problem Statement
  Personas like Priya (boutique owner) and Fatima (food cart operator) rely heavily on in-person sales. Currently, the OHC platform requires them to process online payments or manually record cash. They need a frictionless way to accept physical credit cards and mobile wallets (Apple Pay/Google Pay) directly from their existing smartphones without buying additional bulky hardware. If they cannot quickly tap a customer's card to their phone, they lose sales during busy rushes.

  ## Research Report
  - **Market Context**: Shopify POS, Square, and Stripe all support "Tap to Pay on iPhone" and "Tap to Pay on Android". This capability allows a merchant's standard NFC-enabled smartphone to act as a point-of-sale terminal.
  - **User Need**: Priya needs to manage her online and in-store inventory centrally. When a customer walks into her boutique, she needs to ring up a dress and accept payment immediately via Tap-to-Pay without switching apps or manually syncing inventory later. Fatima needs an ultra-fast checkout process for her food cart queue.
  - **Competitive Analysis**:
    - *Square*: Sets the standard for ease of use but is a closed ecosystem.
    - *Shopify POS*: Great inventory sync but requires users to be fully in the Shopify ecosystem.
    - *Stripe Terminal (Tap to Pay)*: Provides the SDKs necessary to integrate this directly into a mobile app. OHC already uses Stripe for online payments, making this the natural extension.
  - **Feasibility**: Stripe Terminal provides SDKs for both iOS and Android. Since OHC's mobile app is built with Flutter, we can utilize a Flutter wrapper for Stripe Terminal (e.g., `stripe_terminal` package) to enable this functionality natively.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as OHC App (Flutter)
      participant OHC_Backend as OHC Backend (Go)
      participant Stripe as Stripe API

      Owner->>OHC_Backend: Request Connection Token (Tenant ID)
      OHC_Backend->>Stripe: POST /v1/terminal/connection_tokens
      Stripe-->>OHC_Backend: Connection Token
      OHC_Backend-->>Owner: Connection Token

      Owner->>Owner: Initialize Stripe Terminal SDK (Tap to Pay)

      Owner->>OHC_Backend: Create Payment Intent (Amount)
      OHC_Backend->>Stripe: POST /v1/payment_intents
      Stripe-->>OHC_Backend: Client Secret
      OHC_Backend-->>Owner: Client Secret

      Owner->>Owner: collectPaymentMethod() via NFC
      Owner->>Owner: processPayment()
      Owner->>OHC_Backend: Confirm Payment & Record Ledger
      OHC_Backend-->>Owner: Success
  ```

  ### UI Wireframes / Screen Flow Description (375px)
  1. **POS Cart View**: The user selects items from their inventory catalog. A large, prominent "Checkout ($XX.XX)" button sits at the bottom of the screen.
  2. **Payment Method Selection**: A translucent bottom sheet slides up offering "Tap to Pay", "Send Payment Link", or "Cash".
  3. **Tap to Pay Active State**: The screen transitions to a minimalist, high-contrast prompt: "Hold card or phone near top of screen". A native OS overlay for NFC reading appears.
  4. **Success State**: A clear green checkmark animation with a summary of the order and an option to "Email Receipt" or "Text Receipt".

  ### Mobile UX Flow
  - Layout must be extremely simple and easily tappable with one hand (e.g., while holding a bag or product).
  - Touch targets for inventory items and checkout are large (min 44x44px).
  - Error states (e.g., "Card Read Error") must immediately offer a retry button or fallback to "Send Payment Link".
  - Follows OHC Premium Token library with translucent glass materials on overlays.

  ### AI Agent Integration Points
  - **Finance & Decision Assistant**: Logs the successful transaction and updates the daily revenue summary.
  - **Customer & Relationship Assistant**: If the customer taps a card linked to a known profile (via Stripe's customer mapping or manual email entry for receipt), the agent tags the visit and updates purchase history.
  - **Operations Assistant**: Automatically decrements inventory for the purchased items and triggers reorder alerts if stock drops below the threshold.

  ### Key Design Decisions
  - **Stripe Terminal**: Chosen because OHC already leverages Stripe. It provides a unified ledger for both online and in-person payments.
  - **No Hardware Required**: Focusing on "Tap to Pay on iPhone/Android" first, rather than physical Bluetooth card readers, to maintain the "Radical Simplicity" core value.

  ## Implementation Prompt
  Implement the backend API endpoints and the Flutter frontend integration for Stripe Terminal Tap-to-Pay.

  **Acceptance Criteria:**
  1. Create a Go backend endpoint `/api/v1/pos/terminal/connection_token` that returns a Stripe Terminal connection token scoped to the user's tenant.
  2. Create a Go backend endpoint `/api/v1/pos/terminal/create_intent` that creates a Stripe PaymentIntent for in-person payments.
  3. Ensure both endpoints strictly enforce tenant isolation and Zero Trust authentication using the standard OHC auth middleware.
  4. Outline the Flutter implementation required to consume these endpoints and trigger the native Tap-to-Pay UI.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []