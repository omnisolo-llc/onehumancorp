issue_title: "Integrate Stripe Terminal / Tap to Pay for In-Person Payments"
issue_description: |
  # Research Report: Stripe Terminal / Tap to Pay for OHC

  ## Problem Statement
  Owners like **Priya (Boutique Operator)** and **Carlos (Field Service Owner)** conduct business in person. While OHC handles online invoicing and payment links, they currently have to jump to a separate terminal or app to take physical credit cards in their shop or at a customer's house, creating a gap between the work assistant (OHC) and the actual point of sale.
  Priya needs visibility into tap-to-pay transactions natively in her OHC feed so her online demand and in-store operations are unified. Carlos needs a seamless way to collect a deposit immediately after completing a home repair, right from his Android phone without needing extra hardware.

  ## Research & Market Context
  - Competitors like Square, Shopify POS, and Wix inherently unify online sales and in-person POS. OHC, aiming to be the singular work assistant, needs a solution for in-person payments.
  - **Stripe Terminal** offers SDKs for Tap to Pay on Android and iPhone, allowing a mobile device to act as a contactless reader without extra hardware. This is perfect for the OHC Flutter app.
  - Furthermore, Stripe Terminal supports smart readers (like the BBPOS WisePOS E) for permanent counter setups (Priya's boutique).
  - Integrating Stripe Terminal API directly aligns with OHC's existing Stripe reliance and its architecture (Flutter PWA/mobile + Rust backend).
  - The API enables cloud-based routing to physical readers or local handling for Tap to Pay via the mobile SDK.

  ## Design Doc
  - **Integration Target**: Stripe Terminal API.
  - **User Experience**:
    - When generating an invoice, quote, or order, the owner clicks "Collect Payment Now".
    - OHC checks if the user is on a compatible mobile device for Tap to Pay, or if they have a registered physical reader.
    - OHC initiates a Stripe Terminal PaymentIntent.
    - If Tap to Pay on mobile, the Flutter SDK takes over for the NFC read. If a physical reader, the backend pushes the intent to the reader via the Stripe API.
    - Upon success, the payment is captured, the OHC invoice is marked paid, and the event hits the Work Triage feed.
  - **Backend**: Add a Stripe Terminal integration module that can handle `ConnectionTokens` (needed for SDK initialization) and route `PaymentIntents` to physical readers.

  ## Implementation Prompt
  - Create a new backend integration for Stripe Terminal (or extend the existing Stripe integration) in `src/server/integrations/stripe` to generate ConnectionTokens and handle terminal-specific PaymentIntent routing.
  - Update the UI so that when a payment is required, a non-technical owner sees a clear "Collect in-person" or "Tap to Pay" option alongside "Send link".
  - Ensure the backend handles the webhook or callback confirming the in-person payment and updates the corresponding business objects.
  - Add integration to `catalog.rs`.
  - Provide a stub/mock reader flow for E2E testing to prove the CUJ works without real hardware.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
