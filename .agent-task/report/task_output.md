issue_title: "Mobile-First Tap-to-Pay (Stripe Terminal SDK) Integration for OHC POS"
issue_description: |
  **Mission Queue Protocol Brief**

  **Problem Statement:**
  Small business owners (like Priya the boutique owner or Carlos the handyman) need to collect in-person payments seamlessly without purchasing expensive dedicated POS hardware. Currently, OHC POS allows recording cash sales and managing offline inventory, but lacks a native, friction-free way to process credit cards in-person directly on their existing 375px mobile devices. This forces them to juggle multiple apps or buy hardware they don't want, breaking the "One Human Corp" unified assistant promise.

  **Research Report:**
  - **Competitor Analysis:** Shopify POS, Square, and Stripe's own apps offer native Tap-to-Pay on iPhone and Android. They leverage the Stripe Terminal SDK to use the device's NFC chip securely.
  - **Current OHC State:** OHC POS supports optimistic inventory sync and cash sales (as verified in our POS E2E tests and `pos-inventory-sync-optimistic.spec.ts`) but relies on web-only checkout flows for card payments, which are not suitable for fast, in-person checkout.
  - **Opportunity:** Integrating Stripe Terminal SDK (Tap-to-Pay) directly into the Flutter/PWA shell of OHC allows owners to take payments instantly. The Operations Agent can immediately reconcile the payment with the cart and update inventory. This positions OHC as a complete replacement for Square and Shopify POS.

  **Design Doc:**
  - **Architecture Diagram:**
    ```mermaid
    graph TD
        A[OHC Mobile Shell / POS] -->|Initialize Tap-to-Pay| B[Stripe Terminal SDK]
        A --> C[Backend /api/v1/pos/terminal/intent]
        C --> D[Stripe API]
        B -->|Read NFC Card| E[Stripe Payment Intent]
        E -->|Webhook/Callback| F[OHC Payment Service]
        F --> G[Operations Agent Triage]
    ```
  - **Mobile UX Flow (375px):**
    1. Owner adds items to the cart in the POS view.
    2. Owner taps "Collect Payment" -> Selects "Tap to Pay".
    3. The native Tap-to-Pay bottom sheet appears (OS-level UI).
    4. Customer taps card.
    5. Sheet dismisses, showing a large green success checkmark and immediately clearing the cart.
  - **AI Agent Integration Points:**
    - **Finance Assistant:** Instantly logs the deposit and updates the daily revenue summary.
    - **Operations Assistant:** Automatically adjusts inventory (syncing with online storefront) and flags any discrepancies.
  - **Key Design Decisions:**
    - Use Stripe Terminal SDK for maximum compatibility and security without handling raw card data.
    - Wrap the SDK in a Flutter plugin to communicate with the PWA layer if needed, or build the POS shell natively in Flutter.
    - Strictly enforce multi-tenant isolation by using the `tenant_id` associated with the Terminal Location in Stripe.

  **Implementation Prompt:**
  Implement the Stripe Terminal SDK for Tap-to-Pay functionality within the mobile POS flow. Add a "Tap to Pay" payment method to the POS checkout screen. Ensure that when tapped, it triggers the native Stripe Terminal UI to collect payment. The backend must create the corresponding PaymentIntent and capture it, then update the local POS state and inventory. This must work seamlessly on a 375px viewport and follow the macOS Translucent Glass styling. Do NOT prescribe specific database schemas, API endpoints, or function signatures. Let the implementer design those.

  **Priority:** P0

  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
