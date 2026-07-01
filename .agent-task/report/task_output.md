issue_title: "Feature: Zero-Configuration Tap-to-Pay & Mobile POS Architecture"
issue_description: |
  # Mission Queue Protocol: Zero-Configuration Tap-to-Pay & Mobile POS Architecture

  ## Mandatory Phase -1: Live Environment Observation Note
  **Attempted Dogfooding**: The live stack was initiated using `docker compose -f deploy/docker-compose.yml up -d --build`. However, the deployment failed due to sandbox infrastructure constraints (`failed to mount /tmp/containerd-mount... err: invalid argument` related to overlayfs). Because the live product could not be observed directly via Playwright, this architectural design is grounded in the stated personas, codebase analysis (multi-tenant PostgreSQL, gRPC APIs, Go+Bazel backend, Flutter mobile shell), and industry best practices for Mobile POS and Tap-to-Pay functionalities.

  ## Problem Statement
  Priya (boutique owner) needs in-person tap-to-pay visibility alongside her online storefront without complicated hardware setups or technical hurdles. Carlos (handyman) requires a way to collect instant deposits or full payments while on a job site using his Android phone. Currently, OHC lacks a unified, zero-configuration mobile point-of-sale (POS) and Tap-to-Pay (T2P) capability that integrates seamlessly with the centralized inventory, ledger, and customer CRM. Requiring owners to buy external card readers or use a fragmented app breaks the "OneHumanCorp" promise.

  ## Research Report
  - **Market Context**: Stripe recently expanded Tap-to-Pay on iPhone and Android via their Terminal SDK. Shopify's POS Go and Square's dedicated mobile apps lead the market but often feel distinct from their core web dashboards.
  - **Competitor Analysis**:
    - *Square*: Native hardware and T2P functionality are excellent, but the app experience can be cluttered with upsells.
    - *Shopify POS*: Powerful but requires separate configuration and often a separate app download.
  - **Opportunity for OHC**: By integrating Stripe Terminal (Tap-to-Pay) directly into the Flutter OHC mobile shell, we can offer a true "Zero-Configuration" checkout. When Priya or Carlos opens their OHC app, the "Take Payment" button immediately activates the device's NFC reader via native channels (Flutter MethodChannels calling Apple/Google native Tap-to-Pay APIs backed by Stripe), completely invisible to the owner. The payment instantly reflects in the OHC unified ledger and inventory.

  ## Design Doc
  ### High-Level Architecture (Mermaid)
  ```mermaid
  sequenceDiagram
      actor Owner as Priya (Boutique)
      participant App as OHC Flutter App (Mobile)
      participant SDK as Stripe Terminal (Native T2P)
      participant API as OHC Go Backend
      participant DB as Multi-Tenant Postgres

      Owner->>App: Clicks "Take Payment" ($50)
      App->>API: Request Terminal Session Token (tenant_id)
      API->>Stripe: Create ConnectionToken
      Stripe-->>API: ConnectionToken
      API-->>App: ConnectionToken

      App->>SDK: Initialize & Start Tap-to-Pay
      SDK-->>App: Reader Ready (NFC Active)
      Owner->>SDK: Customer taps credit card
      SDK->>Stripe: Process PaymentIntent
      Stripe-->>SDK: Payment Success

      SDK-->>App: Transaction Complete
      App->>API: Record Payment & Decrement Inventory
      API->>DB: Update Ledger & Inventory (tenant_id scoped)
      API-->>App: Sync State
      App-->>Owner: Show "Payment Successful" Screen
  ```

  ### Mobile UX Flow (375px First)
  1. **Home Feed**: Carlos sees today's tasks. He taps a "Service Completed" action card.
  2. **Invoice/Payment Screen**: A clear, large-text screen shows the total amount due. A primary action button: "Tap to Pay".
  3. **NFC Prompt**: The native OS Tap-to-Pay bottom sheet appears over the OHC app (Translucent Glass style native UI).
  4. **Success State**: A green checkmark with haptic feedback. The app transitions back to the OHC receipt screen where Carlos can tap to email or SMS the receipt.

  ### AI Agent Integration
  - **Finance Assistant**: Automatically reconciles the in-person transaction in the daily summary. "You took 3 in-person payments today totaling $150."
  - **Customer Assistant**: If the tapped card matches a known customer profile (via Stripe Customer ID matching), the agent logs the visit to the CRM and can draft a "Thank you for your purchase!" follow-up.
  - **Operations Assistant**: Decrements the purchased item from Priya's inventory instantly to prevent double-selling online.

  ### Key Design Decisions
  - **Flutter Native Integration**: We will use a Flutter plugin that wraps Stripe Terminal iOS/Android SDKs. This ensures we don't need a separate POS app.
  - **Tenant Isolation**: All Terminal connection tokens must be strictly scoped to the `tenant_id` making the request.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your mission is to implement the "Zero-Configuration Tap-to-Pay" feature within the OHC Flutter mobile shell and Go backend.
  1. **Backend**: Implement the gRPC/REST endpoints in Go to generate Stripe Terminal connection tokens for the authenticated tenant. Ensure strict row-level security and `tenant_id` isolation.
  2. **Frontend (Flutter)**: Create the UI flow for taking a payment on a 375px screen. Integrate a Stripe Terminal Flutter plugin to trigger the native Tap-to-Pay UI.
  3. **Verification**: You must write Playwright E2E tests that mock the Stripe Terminal response (since NFC cannot be tested in CI) but fully test the OHC UI flow, backend token generation, and the resulting ledger update.
  4. **Zero Mocks in UI**: The UI must rely entirely on the real backend API for connection tokens and transaction recording.

  **Acceptance Criteria**: A logged-in owner can tap "Take Payment", the app requests a token, "activates" the reader (simulated in tests), processes the payment, and the OHC ledger updates.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, mobile-pos]
assignees: []
