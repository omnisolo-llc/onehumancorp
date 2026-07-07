issue_title: "Implement Stripe Terminal Connection Token Backend Integration"
issue_description: |
  # Research Report: Implement Stripe Terminal Connection Token Backend Integration

  ## Problem Statement
  Currently, OneHumanCorp (OHC) handles online payments efficiently via Stripe Checkout and custom payment routing. However, real-world omni-channel personas like Priya (the boutique owner) frequently transact in person. Priya needs to seamlessly ring up customers in her store using her phone (Tap to Pay) or a dedicated card reader. The lack of a native in-person Point of Sale (POS) solution forces users to rely on disjointed third-party systems, breaking the "all-in-one" OHC promise.

  ## Research Report
  - **OHC Gaps:** A review of `src/server/integrations/stripe/` reveals implementations for Checkout, Subscription, and Routing (Card vs. ACH, Razorpay, MercadoPago). The file `terminal.rs` exists but needs to implement the endpoint to generate Stripe Terminal connection tokens.
  - **Competitor Landscape:**
    - *Shopify:* Offers a robust, deeply integrated POS system with dedicated hardware and Tap to Pay on iPhone/Android, seamlessly syncing online and offline inventory.
    - *Wix & Squarespace:* Provide POS integrations, though often as add-ons, blurring the lines between online storefront and physical retail.
  - **Opportunity:** Integrating Stripe Terminal will enable OHC users to accept in-person payments directly within the OHC app, utilizing Tap to Pay on compatible devices or connecting to Stripe readers.

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

  ### AI Agent Integration Points
  - *Finance & Payments ("The Accountant"):* Automatically reconciles in-person transactions with online sales, generating unified daily reports.
  - *Operations ("The Manager"):* Instantly deducts sold items from inventory, triggering low-stock alerts if necessary.

  ### Key Design Decisions
  - **Connection Tokens:** Backend must expose a secure method to generate Stripe Terminal connection tokens scoped to the specific tenant.
  - **Multi-Tenancy:** Ensure strict isolation so a connection token is inextricably linked to the user's `tenant_id`.

  ## Implementation Prompt
  As an implementer, your task is to integrate Stripe Terminal Connection Token generation into OHC's backend.
  1. Implement the `create_connection_token` logic in `src/server/integrations/stripe/terminal.rs`.
  2. The function should make an HTTP POST request to `https://api.stripe.com/v1/terminal/connection_tokens`.
  3. Ensure the function correctly sets the Authorization header with the provided Stripe API key.
  4. Ensure proper error handling and logging.
  5. Add unit tests in `src/server/integrations/stripe/terminal.rs` to verify the connection token generation logic (mocking the HTTP client or using a test API key if available/appropriate in the codebase).
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
