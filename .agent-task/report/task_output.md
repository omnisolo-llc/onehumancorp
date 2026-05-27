issue_title: "Implement Shippo Multi-Carrier Shipping Integration"
issue_description: |
  ## Problem Statement
  Small business owners selling physical goods face immense friction setting up shipping. Terms like "zones," "carrier accounts," and "negotiated rates" are intimidating, and manually copying tracking numbers from carrier sites back into an online store is tedious and error-prone. Our non-technical users need a seamless way to see multi-carrier rates, buy and print labels instantly, and automatically update customers with tracking information, all without leaving the OHC platform.

  ## Research Report
  Shippo emerged as the standout candidate for solving OHC’s multi-carrier shipping gap.

  **Capabilities & Features**:
  *   **Multi-Carrier Access:** Shippo provides built-in connections to over 40 global carriers (USPS, UPS, FedEx, DHL Express, Canada Post, etc.). It removes the need for merchants to negotiate individual contracts.
  *   **Deep Discounts:** Up to 90% savings on standard rates out-of-the-box via their default API endpoints.
  *   **Key Functionality:** API endpoints cover address validation (domestic and international), rate comparison, label generation (including batch generation and return labels), shipping insurance, customs forms for international orders, and real-time tracking webhooks.

  **Target Personas & Usability**:
  *   Perfect for our operational personas. This solves the "Setup Complexity" and "Operational Fatigue" pain points identified in our SMB Pain Points audit. By integrating Shippo under the hood, OHC users won't have to manage a separate SaaS dashboard for fulfillment.

  **Pricing Model**:
  *   Shippo offers a generous free tier ("Starter") with no monthly fee, charging only a nominal 5¢ per label if connecting an external carrier account, or $0 if using Shippo's discounted carrier accounts. Subscriptions start at $17/mo for Pro (branded tracking, more users). Their pay-as-you-go and SaaS API pricing scales excellently for our Multi-tenant Cloud and Standalone desktop modes.

  **Architecture Compatibility (Cloud & Standalone)**:
  *   Shippo provides standard REST APIs.
  *   **Cloud Mode:** Can handle high volume webhook updates for tracking across all OHC tenants.
  *   **Standalone Mode:** The API is stateless; our local Rust/SQLite setup can interface with the API securely by storing the Shippo API key locally, allowing offline-first merchants to sync shipping data upon internet reconnection.

  ## Design Doc
  **Trigger**: When an order is placed and ready for fulfillment, the merchant accesses the "Fulfill Order" module within the OHC interface.
  **Actions Taken**:
  1.  **Address Validation**: OHC hits the Shippo Address API to silently validate the buyer's shipping address.
  2.  **Rate Fetching**: The integration queries the Shippo Rating API using the order's weight/dimensions and returns the 3 best options in plain language.
  3.  **Label Generation**: Upon selection, the OHC backend requests the label via the Shippo API. The label PDF is stored, and the tracking number is synchronized.
  4.  **Customer Notification**: A background worker listens for Shippo tracking webhooks and triggers the OHC unified inbox to send an automated "Order Shipped!" email/SMS to the customer.
  **User Experience**: A pure, jargon-free visual interface where they click "Buy Label for $4.32", the label prints, and the order auto-updates to 'Fulfilled'.

  ## Implementation Prompt
  Implement the Shippo integration layer focusing on a frictionless, zero-configuration experience for the merchant.

  **Acceptance Criteria:**
  *   A new merchant can fulfill an order and generate a shipping label without ever visiting goshippo.com or pasting API keys manually.
  *   During fulfillment, present the user with simple, plain-language rate choices (e.g., "$5.50 - Arrives by Friday").
  *   Successfully store the generated label artifact and tracking number in the local or cloud database.
  *   Automatically fire a notification to the customer when the tracking status changes to 'Shipped' or 'Delivered'.
  *   Adhere to the OHC "Glassmorphism" visual excellence mandate for all fulfillment UI elements.

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
