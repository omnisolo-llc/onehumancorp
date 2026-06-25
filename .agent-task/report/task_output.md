issue_title: "Architecture Design: Mobile-First Tap-to-Pay (NFC) Integration for In-Person Operations"
issue_description: |
  # Mobile-First Tap-to-Pay (NFC) Architecture for OneHumanCorp

  ## Problem Statement
  Our primary business personas—Carlos (handyman, working on-site from an Android phone) and Priya (boutique operator, taking in-person payments alongside online orders)—currently lack a seamless, integrated way to collect in-person payments directly on their mobile devices without relying on clunky third-party external card readers or manual entry. When Carlos finishes a repair or Priya rings up a customer in-store, they need to instantly accept credit cards via NFC (Tap-to-Pay) on their phone, ensuring the payment syncs perfectly with their OHC inventory, bookings, and ledger. A disjointed payment experience causes friction, delays revenue collection, and requires the owner to reconcile systems manually, defeating the purpose of OHC as their single unified work assistant.

  ## Research Report
  ### Track 1: Architectural Gap & Scaling Discovery
  - **Codebase Audit:** OHC currently possesses Stripe integrations for online checkout sessions and payment intents (e.g., invoice payments, booking deposits), but lacks the `Stripe Terminal` SDK and infrastructure required to transform an owner's mobile device into an NFC reader. Multi-tenant rules are established for digital payments but not for physical, location-bound point-of-sale (POS) sessions.
  - **Competitor Analysis:**
    - **Shopify POS:** Offers excellent hardware integrations and its own Tap-to-Pay on iPhone, but heavily pushes merchants toward purchasing dedicated Shopify POS hardware. The setup is overkill for a handyman like Carlos.
    - **Square:** The undisputed king of SME in-person payments. However, Square's software ecosystem outside of payments and scheduling is fragmented. Their Tap-to-Pay on mobile is frictionless, which is exactly the standard we must meet or beat.
    - **Stripe Tap to Pay:** The underlying API we will use. It enables iPhones and Androids to act as contactless readers natively without extra hardware.
  - **The Gap:** OHC needs a zero-hardware, mobile-first Tap-to-Pay flow integrated directly into the OHC Flutter app. This closes the loop for omni-channel personas, bridging the gap between digital invoicing and physical interactions.

  ## Design Doc
  ### Track 2: Selected Architecture Deep Dive & Track 3: Technical Integrity

  #### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      actor Owner as Carlos/Priya (OHC App)
      participant OHC_Flutter as Flutter App (Terminal SDK)
      participant OHC_Backend as Go Backend (Terminal API)
      participant Stripe as Stripe Terminal API
      participant Ledger as OHC Multi-Tenant Database

      Owner->>OHC_Flutter: Taps "Collect Payment (NFC)"
      OHC_Flutter->>OHC_Backend: Request ConnectionToken(location_id)
      OHC_Backend->>Stripe: POST /v1/terminal/connection_tokens
      Stripe-->>OHC_Backend: connection_token
      OHC_Backend-->>OHC_Flutter: connection_token
      OHC_Flutter->>Stripe: Init SDK & Discover local reader
      Stripe-->>OHC_Flutter: Local Reader Connected
      OHC_Flutter->>OHC_Backend: Create PaymentIntent (Amount, OrderID)
      OHC_Backend->>Stripe: POST /v1/payment_intents (terminal capture)
      Stripe-->>OHC_Backend: client_secret
      OHC_Backend-->>OHC_Flutter: client_secret
      OHC_Flutter->>Stripe: CollectPaymentMethod(client_secret)
      Owner->>Owner: Present phone to Customer for Tap
      Stripe-->>OHC_Flutter: PaymentMethod Collected
      OHC_Flutter->>OHC_Backend: ProcessPayment(PaymentIntent_ID)
      OHC_Backend->>Stripe: POST /v1/payment_intents/{id}/capture
      Stripe-->>OHC_Backend: Success (Payment Confirmed)
      OHC_Backend->>Ledger: Insert Ledger Entry & Update Order Status
      OHC_Backend-->>OHC_Flutter: Payment Successful
  ```

  #### Mobile UX Flow & UI Wireframes (375px baseline)
  - **Screen 1: Order/Invoice Summary (375px)**
    - Clean, UniFi-style modular card displaying the total amount due, customer name, and a list of items/services.
    - Primary Action (Floating or sticky bottom): Translucent glass button "Tap to Pay • $150.00".
  - **Screen 2: Tap to Pay Initialization Modal**
    - A smooth bottom-sheet modal slides up.
    - Status text: "Connecting to secure payment network..." (Connecting to Stripe Terminal SDK).
    - If location services/Bluetooth permissions are missing, present a one-tap grant button.
  - **Screen 3: Native NFC Prompt**
    - The OS-level Tap-to-Pay overlay appears (Apple Tap to Pay or Android equivalent).
    - The owner presents the phone to the customer. The customer taps their card or phone.
  - **Screen 4: Success & Next Action**
    - A crisp, green success checkmark with haptic feedback.
    - Assistant-generated text: "Payment complete. Should I email the receipt to customer@email.com?"
    - Buttons: "Send Receipt" or "Done".

  #### AI Agent Integration Points
  - **Sales & Revenue Assistant:** Automatically logs the transaction, updates the daily revenue metric, and removes the "Unpaid" flag from the customer's profile.
  - **Customer Relationship Assistant:** Drafts a personalized "Thank you for your business!" follow-up message with the digital receipt attached, prompting the owner to send it instantly.

  #### Key Design Decisions & Why
  - **No External Hardware:** By strictly utilizing the native Tap-to-Pay (Stripe Terminal mobile SDKs), we enforce the OHC core value of "Radical Simplicity." Owners do not need to order, charge, or pair external Bluetooth dongles.
  - **Backend-Driven Intent Creation:** PaymentIntents are created securely on the Go backend to enforce multi-tenant isolation and pricing rules. The Flutter client only handles the physical card collection mechanism, maintaining Zero Trust.
  - **Offline-Tolerant UI:** The initiation screen must handle poor connection gracefully. If network fails before intent creation, provide a human-readable error ("Can't reach the payment network right now. Try moving to better service.").

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to integrate the `stripe_terminal` Flutter SDK and build the Go backend endpoints to support mobile-first Tap-to-Pay.

  - **CUJ:** An owner (like Carlos) opens an unpaid invoice in the mobile app, taps "Collect Payment," and successfully processes a contactless credit card transaction using only their phone.
  - **Acceptance Criteria:**
    1. Implement Go endpoints to vend Stripe Terminal connection tokens based on the tenant's registered location.
    2. Implement the Flutter UI integration using the Translucent Glass design tokens for the "Tap to Pay" flow.
    3. The app must correctly request and handle location/NFC permissions natively.
    4. Upon successful payment, the backend must transition the order/invoice state to paid and create a ledger entry.
    5. Write Playwright E2E tests (using Stripe's simulated test readers) verifying the end-to-end checkout flow from the 375px mobile viewport.
    6. Ensure multi-tenant isolation is strictly enforced via `tenant_id` context in all backend database mutations.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []