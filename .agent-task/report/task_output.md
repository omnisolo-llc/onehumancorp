issue_title: "[Architecture] Stripe Terminal POS Architecture for In-Person Payments"
issue_description: |
  # Research Report: Implement Stripe Terminal POS Architecture for In-Person Payments

  ## Problem Statement
  Currently, OneHumanCorp (OHC) handles online payments efficiently via Stripe Checkout and custom payment routing. However, our real-world personas, like Priya (the boutique owner) and Carlos (the handyman), frequently transact in person. Priya needs to seamlessly ring up customers in her store using her phone (Tap to Pay) or a dedicated card reader, while Carlos needs to accept final payments on-site after a repair. The lack of a native in-person Point of Sale (POS) solution forces users to rely on disjointed third-party systems, breaking the "all-in-one" OHC promise and muddying financial reporting.

  ## Research Report
  - **OHC Gaps:** A review of `src/server/integrations/stripe/` reveals implementations for Checkout, Subscription, and Routing (Card vs. ACH, Razorpay, MercadoPago), but zero support for Stripe Terminal. We need to expose API endpoints in `src/server/api/` that delegate to the client in `src/server/integrations/stripe/client.rs`. The grpc endpoints are already partially implemented in `src/server/lib.rs` for `HubService` but they need REST equivalents in `src/server/api/`. We also need to add support for creating Payment Intents that can be captured later.
  - **Competitor Landscape:**
    - *Shopify:* Offers a robust, deeply integrated POS system with dedicated hardware and Tap to Pay on iPhone/Android, seamlessly syncing online and offline inventory.
    - *Wix & Squarespace:* Provide POS integrations, though often as add-ons, blurring the lines between online storefront and physical retail.
  - **Opportunity:** Integrating Stripe Terminal will enable OHC users to accept in-person payments directly within the OHC app, utilizing Tap to Pay on compatible devices or connecting to Stripe readers (e.g., BBPOS WisePad 3, Stripe Reader S700). This provides a unified ledger and centralized inventory management.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
    Client[OHC Mobile App - POS View] -->|Request Connection Token| API[OHC Backend API]
    API -->|Create ConnectionToken| Stripe[Stripe API]
    Stripe -->|Token| API
    API -->|Token| Client
    Client -->|Initialize Terminal SDK| Reader[Stripe Reader / Tap to Pay]
    Reader -->|Read Card Data| Client
    Client -->|Create PaymentIntent| API
    API -->|Confirm PaymentIntent| Stripe
    Stripe -->|Capture Success| API
    API -->|Update Ledger & Inventory| DB[(PostgreSQL)]
    API -->|Notify Success| Client
  ```

  ### UI Wireframes
  - A "POS / In-Person" tab in the Operations dashboard.
  - A clean, calculator-like keypad or product selection grid (synced with the master catalog).
  - A "Charge $X.XX" button that triggers the Terminal SDK overlay (Tap to Pay) or connects to a paired Bluetooth reader.
  - Glassmorphism design elements (translucent overlays, clean typography) adhering to OHC's visual standards.

  ### Mobile UX Flow
  - The app layout scales properly to a 375px width, ensuring that all components fit perfectly on smaller screens.

  ### AI Agent Integration Points
  - *Finance & Payments ("The Accountant"):* Automatically reconciles in-person transactions with online sales, generating unified daily reports.
  - *Operations ("The Manager"):* Instantly deducts sold items from inventory, triggering low-stock alerts if necessary.

  ### Key Design Decisions
  - **Connection Tokens:** Backend must expose an endpoint (`/api/v1/payments/terminal/token`) to securely generate Stripe Terminal connection tokens scoped to the specific tenant.
  - **Payment Intents:** POS flow will use server-side PaymentIntents, captured once the physical card is read by the Terminal SDK.
  - **Multi-Tenancy:** Ensure strict isolation so a connection token or PaymentIntent is inextricably linked to the user's `tenant_id`.

  ## Implementation Prompt
  As an implementer, your task is to integrate Stripe Terminal into OHC's payment architecture.
  1. Add a new `terminal.rs` module under `src/server/integrations/stripe/` to handle `ConnectionToken` generation and Stripe Terminal specific API interactions. Update `src/server/integrations/stripe/mod.rs` and `BUILD.bazel`. Move the connection token code from `client.rs` to this module.
  2. Create secure REST API endpoints in `src/server/api/` for the mobile client to request connection tokens (`/api/v1/payments/terminal/token`).
  3. Ensure the `PaymentIntent` creation flow supports in-person capture methods.
  4. The end result should allow a user (like Priya) to open the OHC app, request a token, and begin a payment intent.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
