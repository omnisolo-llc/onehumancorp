issue_title: "Architectural Design: Universal Mobile Tap-to-Pay & Agentic In-Person POS Integration"
issue_description: |
  ## 1. Problem Statement
  In-person operators like Priya (Boutique Operator), Carlos (Field Service), and Fatima (Food Cart) need to accept payments in the physical world without buying expensive, clunky hardware terminals. Currently, they have to rely on third-party POS apps (like Square or a separate Stripe terminal app) that do not sync in real-time with their central OHC inventory, customer profiles, or Operations Agent tasks. This creates fragmented data, manual reconciliation headaches, and double-selling inventory risks.

  ## 2. Research Report
  - **Market Context**: Square dominates the micro-merchant in-person payment space precisely because they unified hardware and software. Shopify offers a robust POS, but it requires dedicated hardware or a separate heavy POS app. Stripe now offers Terminal SDKs with "Tap to Pay on iPhone/Android," allowing the merchant's everyday phone to act as the contactless reader without extra hardware.
  - **The OHC Opportunity**: By natively integrating Stripe's Tap to Pay SDK into the OHC Flutter mobile app, we can turn every owner's phone into a payment terminal instantly. Crucially, because it's unified with OHC, in-person sales instantly update central inventory and trigger agentic workflows (e.g., the Marketing Agent captures an in-person buyer's email for a digital receipt and schedules a follow-up discount).
  - **Competitor Gaps**:
    - *Square*: Hardware-centric, weaker on complex online agentic follow-ups.
    - *Shopify POS*: Heavy, separate app, overkill for a handyman or food cart.
    - *Wix POS*: Limited geographical support and fragmented inventory updates.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Owner (OHC App)
      participant OHC as OHC Flutter App (Tap to Pay SDK)
      participant Backend as OHC Backend (Go/Bazel)
      participant Stripe as Stripe Terminal API
      participant Agent as Sales/Ops Agent

      Owner->>OHC: Initiates In-Person Sale
      OHC->>Backend: Request PaymentIntent (amount, currency)
      Backend->>Stripe: Create PaymentIntent
      Stripe-->>Backend: client_secret
      Backend-->>OHC: Return client_secret & PaymentIntent
      OHC->>OHC: Activate Tap to Pay Reader
      Owner->>OHC: Customer Taps Card on Phone
      OHC->>Stripe: Process Payment via SDK
      Stripe-->>OHC: Payment Success
      OHC->>Backend: Confirm Payment & Update Inventory
      Backend->>Agent: Trigger Post-Sale Follow-up (Receipt, Loyalty)
  ```
  ### Mobile UX Flow (375px)
  1. **Quick Sale View**: A prominent, always-accessible "Charge" button on the mobile dashboard.
  2. **Amount & Cart**: A minimalist numpad for quick amounts or a fast catalog selector for specific items.
  3. **Tap to Pay Interface**: Native OS-level NFC prompt (Apple Tap to Pay or Android equivalent) appears without leaving the OHC app.
  4. **Post-Sale Action**: Immediate digital receipt option (QR code or enter email/phone).

  ### AI Agent Integration Points
  - **Sales & Revenue Assistant**: Reconciles the in-person payment with daily digital revenue, providing a unified plain-language daily summary.
  - **Customer Relationship Assistant**: If the customer provides an email for the receipt, the agent links the transaction to an existing customer profile or creates a new one, drafting a personalized "thank you" email.

  ## 4. Implementation Prompt
  **Feature Name**: Universal Native Tap-to-Pay & In-Person POS
  **Target Persona**: Priya (Boutique Operator) and Carlos (Field Service)

  **Outcome**: Priya and Carlos can accept contactless credit card payments directly on their mobile phones using the OHC app, without extra hardware. The transaction instantly updates OHC inventory, revenue dashboards, and customer profiles.

  **Critical User Journey (CUJ)**:
  1. Owner opens the OHC mobile app and taps "Quick Charge".
  2. Owner enters an amount or selects an inventory item.
  3. Owner taps "Accept Contactless Payment". The native Tap to Pay UI appears.
  4. Customer taps their physical credit card or phone against the owner's phone.
  5. Payment succeeds, inventory decrements, and a unified receipt screen is shown.

  **Acceptance Criteria**:
  - Integrate Stripe Terminal SDK (Tap to Pay on iPhone/Android) within the Flutter application.
  - Implement a `PaymentIntent` generation and capture flow on the Go backend that associates the charge with the specific tenant and location.
  - Ensure real-time inventory decrement in PostgreSQL with row-level tenant isolation.
  - Implement the 375px-optimized mobile UI for the POS view with clear, high-contrast typography and large touch targets (44x44px minimum).
  - Provide graceful degradation and offline-tolerant caching for catalog browsing if the network is flaky.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
