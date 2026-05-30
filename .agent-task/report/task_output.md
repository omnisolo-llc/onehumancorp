issue_title: "[Architectural Gap] Missing Stripe Terminal Tap-to-Pay Capabilities for In-Person Operations"
issue_description: |
  ## Problem Statement
  OneHumanCorp's core mission is to serve small businesses like **Priya (The Boutique Owner)** who operate in both physical and digital spaces. Currently, OHC heavily relies on Stripe Checkout and Payment Links, which are great for online transactions (Maya the Baker, Leo the Music Tutor) but severely limit in-person sales scenarios.

  Priya needs a unified point-of-sale (POS) experience that syncs her in-person inventory with her online storefront. Without Stripe Terminal (Tap-to-Pay) integration, she is forced to use external hardware or alternative apps, fracturing her business data, operations, and breaking OHC's "All-in-one" promise.

  ## Research Report
  - **Competitor Landscape**:
    - **Shopify**: Offers a fully integrated POS system that syncs directly with online inventory and supports tap-to-pay via mobile apps and dedicated hardware. This is a primary reason retailers choose Shopify.
    - **Square**: Built entirely around in-person hardware and tap-to-pay functionality, now extending into online presence.
    - **Stripe**: Offers **Stripe Terminal**, which provides SDKs and APIs for building custom POS experiences and enabling Tap-to-Pay on compatible iOS and Android devices, without needing extra hardware (Tap to Pay on iPhone/Android).
  - **OHC Architecture Gap**:
    - The `server/integrations/stripe` module currently only supports `checkout_session`, `subscription`, `invoices`, and `payouts`.
    - Missing capabilities include: `TerminalConnectionToken` generation, `PaymentIntent` creation mapped to Terminal, and event handling for physical hardware interactions.
    - Our multi-tenant architecture needs a secure way to distribute short-lived tokens to the mobile client (Flutter app) to initialize the Stripe Terminal SDK for a specific tenant's Stripe connected account.

  ## Design Doc
  ### Architectural Additions
  1.  **Backend Integration**:
      -   Add support for generating Stripe Terminal Connection Tokens.
      -   Add support for creating `PaymentIntents` specifically tailored for Terminal capture (requiring `payment_method_types: ['card_present', 'interac_present']`).
  2.  **API Layer**:
      -   New endpoints under a secure route that validates the user's tenant context and requests a connection token from Stripe on behalf of that tenant.
  3.  **Frontend/Mobile Prep**:
      -   Ensure the API returns exactly what the future Flutter Stripe Terminal SDK needs to initialize the tap-to-pay session.

  ### Security & Multi-Tenancy
  -   Connection tokens must be strongly bound to the `tenant_id`. If OHC uses Stripe Connect, the token must be generated for the specific connected account of that tenant.
  -   The API must ensure that a user belonging to Tenant A cannot request a terminal connection token for Tenant B.

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant App as Mobile App (Priya)
      participant API as OHC API Layer
      participant Stripe as Stripe API
      participant Hardware as Customer Device / Card

      App->>API: 1. Request Connection Token (tenant_id)
      API->>Stripe: 2. Generate Terminal Connection Token
      Stripe-->>API: 3. Token
      API-->>App: 4. Token

      App->>App: 5. Initialize Stripe Terminal SDK with Token
      App->>Stripe: 6. Create PaymentIntent (card_present)
      Stripe-->>App: 7. Intent ID & Client Secret
      App->>Hardware: 8. Prompt Tap-to-Pay on iPhone/Android
      Hardware-->>App: 9. Read Card Data
      App->>Stripe: 10. Process PaymentIntent
      Stripe-->>App: 11. Success
      App->>API: 12. Notify Payment Success (Update DB)
  ```

  ### Mobile UX Flow (375px)
  1.  **Priya** selects an item from her inventory on the OHC app.
  2.  She taps "Charge $45.00 In-Person".
  3.  The OHC app requests a Terminal Connection Token from the backend.
  4.  The backend validates Priya's session, asserts her `tenant_id`, and calls the Stripe API to generate the token.
  5.  The app receives the token, initializes the Stripe Terminal SDK, and brings up the native "Tap to Pay on iPhone/Android" sheet.
  6.  The customer taps their card/phone. The app captures the `PaymentIntent`.
  7.  The app notifies the OHC backend of the successful capture to update inventory and the ledger.

  ### AI Agent Integration
  -   **Finance & Payments (The Accountant)**: Tracks these in-person transactions seamlessly alongside online ones, updating daily revenue reports.
  -   **Operations (The Manager)**: Instantly decrements the local inventory variant count when the POS transaction completes.

  ## Implementation Prompt (For Implementer Agent)
  Implement the backend infrastructure for Stripe Terminal Tap-to-Pay.
  1.  Extend the Stripe client to include API calls for creating Terminal Connection Tokens and Terminal-specific Payment Intents.
  2.  Create a new API route to securely expose the Connection Token generation to authenticated mobile clients, strictly enforcing `tenant_id` isolation.
  3.  Write comprehensive unit tests for the new Stripe client methods.
  4.  Ensure all code passes `bazel test //...` and conforms to the repository's strict formatting and safety standards. Do not prescribe the exact HTTP client specifics, rely on the existing patterns in the Stripe module.

  **Acceptance Criteria**:
  - The backend can successfully request and return a mocked or real Stripe Terminal Connection Token.
  - The API endpoints are protected and correctly scope requests to the authenticated tenant.
  - Test coverage for the new module is 100%.

  **Estimated Scope**: Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
