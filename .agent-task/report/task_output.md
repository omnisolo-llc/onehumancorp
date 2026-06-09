issue_title: "Architecture: Tap-to-Pay Mobile POS Integration for In-Person Payments"
issue_description: |
  ## Title
  Architecture: Tap-to-Pay Mobile POS Integration for In-Person Payments

  ## Problem Statement
  Small business owners like Priya (boutique operator) and Fatima (food cart operator) rely heavily on in-person sales. Currently, OneHumanCorp (OHC) handles online orders, bookings, and digital payments well, but lacks a native in-person checkout experience. Without a built-in Point of Sale (POS) capability, users are forced to rely on external hardware terminals or separate apps (like Square or standalone Stripe Terminal apps) to process physical cards or Apple/Google Pay. This breaks the unified operations promise, leading to disconnected inventory (Priya selling an item in-store that remains available online), fragmented revenue reporting, and a disjointed customer experience. We need an integrated Tap-to-Pay solution that turns the owner's mobile device into a POS terminal, fully synced with OHC's backend and AI assistants.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify POS:** Offers native POS integration. Their solution requires users to download a separate POS app and use card readers. They have recently integrated Apple's Tap to Pay on iPhone.
  - **Square:** The dominant player in mPOS. Started with dongles, now strongly pushes Tap to Pay on iPhone/Android, eliminating the need for extra hardware. Square's ecosystem ties POS directly to inventory and online sales.
  - **Stripe Terminal:** Provides SDKs for both iOS and Android to support Tap to Pay natively, allowing platforms to embed POS directly into their own apps without redirecting to a separate application.
  - **OHC Opportunity:** By integrating Stripe Terminal SDK (specifically Tap to Pay) into the OHC Flutter app, we can provide Priya and Fatima with a seamless checkout flow on their existing devices. This eliminates hardware costs and keeps all transactions, inventory deductions, and customer data strictly within the OHC tenant boundary. Furthermore, the OHC Finance Assistant can instantly reconcile these payments alongside digital revenue, and the Operations Assistant can deduct inventory in real-time.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Flutter Mobile App] -->|Initialize POS Session| B(OHC Backend API)
      B -->|Create ConnectionToken| C[Stripe API]
      C -->|Token| B
      B -->|Token| A
      A -->|Tap to Pay SDK| D[NFC Reader / Customer Card]
      D -->|Payment Intent| A
      A -->|Confirm Payment| B
      B -->|Capture| C
      C -->|Webhook: payment_intent.succeeded| E[OHC Webhook Gateway]
      E --> F[Event Bus / Queue]
      F --> G[Order & Inventory Service]
      F --> H[Finance AI Assistant]
      G -->|Update DB| I[(PostgreSQL Tenant DB)]
  ```

  ### Mobile UX Flow (375px First)
  1. **Cart & Checkout:** Priya adds products to the cart from the OHC mobile catalog. She taps "Charge $45.00".
  2. **Payment Method Selection:** A bottom sheet presents payment options: "Tap to Pay", "Send Invoice", "Cash". She selects "Tap to Pay".
  3. **Tap to Pay UI:** The app invokes the native iOS/Android Tap to Pay interface (provided by Stripe SDK). The screen displays a clear, native NFC prompt.
  4. **Customer Action:** The customer taps their phone or contactless card against Priya's device.
  5. **Confirmation & AI Action:** The screen transitions to a success checkmark. The OHC Operations Assistant immediately flashes a toast: "Inventory updated. Receipt sent to customer."

  ### AI Agent Integration Points
  - **Finance & Decision Assistant:** Listens to successful POS payment events via the event bus to update the daily dashboard and categorize the revenue as "In-Store".
  - **Operations Assistant:** Automatically handles inventory deduction and checks if stock is running low, drafting a reorder reminder if necessary.
  - **Customer Success Assistant:** If the customer is recognized (e.g., via digital receipt email match), updates their omnichannel graph and drafts a personalized "Thank you for visiting the store" message for approval.

  ### Key Design Decisions
  - **Stripe Terminal Tap to Pay:** We will leverage Stripe's Terminal SDK over building a custom EMV kernel, drastically reducing compliance (PCI, EMVCo) scope and time-to-market.
  - **Unified Checkout State:** The cart state and order creation must be identical for online and in-store purchases up until the payment capture phase, ensuring a single source of truth for inventory and revenue.
  - **Offline/Flaky Network Gracefulness:** While Tap to Pay requires connectivity for authorization, the UI must gracefully handle network drops, caching the cart state so the owner doesn't lose the transaction context if they have to step outside for a better signal.

  ## Implementation Prompt
  **User-Facing Outcome:** As an owner like Priya or Fatima, I want to use my own smartphone to securely accept contactless card payments from my customers in person, so that I don't have to buy extra hardware and my in-store sales automatically sync with my online inventory and daily reports.

  **CUJ & Acceptance Criteria:**
  1. Define a gRPC/REST endpoint in the Rust backend (`/api/v1/pos/connection_token`) to generate a Stripe Terminal ConnectionToken scoped to the tenant's Stripe connected account.
  2. Define the unified Order and PaymentIntent flow for POS transactions, ensuring the `source` is marked as `in_person`.
  3. (Mobile Implementation Deferred, but API must support it): Provide an endpoint to handle POS payment intent capture and link it to the local Cart/Order ID.
  4. Ensure backend event listeners for `payment_intent.succeeded` correctly deduct inventory for physical items and emit metrics for the Finance Assistant.
  5. Write E2E Playwright tests simulating the POS order creation and backend payment confirmation flow (using Stripe test mode tokens or mock responses where appropriate for external boundaries).

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
