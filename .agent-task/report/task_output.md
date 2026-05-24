issue_title: "Autonomous Mobile Pass & Digital Wallet Engine"
issue_description: |
  # Title: Autonomous Mobile Pass & Digital Wallet Engine

  ## Problem Statement
  Small business owners (SMBs) struggle with customer retention and offline engagement. Physical loyalty cards get lost, appointment reminders (SMS/Email) get buried, and order pickups at busy times (like Fatima's food cart rush) lead to confusion and delays. Existing platforms rely on the user installing yet another app or digging through emails to find QR codes. For non-technical owners, integrating with native Apple Wallet or Google Wallet is prohibitively complex, requiring specialized developers, certificate management, and API integrations they do not understand. They need a zero-configuration way to put their business directly into their customers' native digital wallets.

  ## Research Report
  *   **Competitor Analysis**:
      *   **Shopify/Wix**: No native, out-of-the-box integration with Apple/Google Wallet for loyalty or appointments. Requires expensive third-party apps (like PassKit or LoopyLoyalty) which introduce friction and additional monthly costs.
      *   **Square**: Offers loyalty, but relies heavily on SMS or the Cash App ecosystem, not universal native wallet passes.
  *   **The OHC Differentiator**: OHC will automatically generate and deliver native digital wallet passes (Apple Wallet & Google Pay) as a core capability. This requires zero configuration from the business owner. The pass acts as a dynamic, real-time connection to the customer, updatable over the air via the Hybrid Event Mesh.

  ## Design Doc

  ### High-Level Architecture
  ```mermaid
  graph TD;
      EventMesh[Hybrid Event Mesh] -->|Trigger: New Order/Booking/Loyalty| WalletAgent[AI Digital Wallet Agent];
      WalletAgent --> PassGenerator[Pass Generation Engine];
      PassGenerator -->|Sign & Package| CryptoSigner[Zero-Trust Signing Service];
      CryptoSigner --> EdgeCDN[Edge Delivery & Caching];
      EdgeCDN -->|SMS/Email/QR| CustomerDevice[Customer Mobile Device];
      CustomerDevice -->|Native Wallet Integration| AppleGoogle[Apple/Google Wallet];
      AppleGoogle -->|Push Updates| PushService[OHC Push Notification Gateway];
      PushService --> EventMesh;
  ```

  ### Key Design Decisions & Invariants
  *   **Zero-Config Generation**: The AI Digital Wallet Agent automatically designs the pass using the business's existing design tokens (colors, logo) without requiring the owner to use a pass builder UI.
  *   **Dynamic Updates**: Passes are living documents. A booking pass updates its time if changed; an order pass updates its status to "Ready for Pickup" natively on the lock screen via Apple/Google Push Notification Services (APNs/FCM).
  *   **Tenant Isolation & Security**: The Zero-Trust Signing Service ensures each tenant's passes are cryptographically isolated. OHC acts as the centralized certificate holder (for Apple/Google) but provisions tenant-specific passes securely.
  *   **Cross-Department Coordination**:
      *   *Marketing Agent*: Uses passes for geo-fenced push notifications (e.g., "You're near Priya's Boutique, 10% off today!").
      *   *Operations Agent*: Updates order status passes for Fatima's food cart.
      *   *Sales Agent*: Updates booking passes for Leo's tutoring sessions.

  ### Mobile UX Flow (375px First)
  1.  **Customer Checkout/Booking (Acquisition)**: After checkout on the OHC Storefront, a prominent native button appears: "Add to Apple Wallet" / "Save to Google Pay".
  2.  **Pass View (Native OS)**: The pass natively matches the OS guidelines. It features a QR code (for Fatima to scan via the Mobile-First Inventory Scanner) and dynamic text fields (Order #, Loyalty Points).
  3.  **Owner View (OHC App)**: The owner sees *no complex wallet settings*. When Fatima marks an order "Preparing" via her KDS, a single green checkmark confirms "Customer notified". The KAIROS Orchestrator handles the wallet push invisibly.
  4.  **Proactive Engagement (Retention)**: When a customer walks within 100 meters of Priya's physical store, the lock screen displays a subtle notification from the wallet pass: "Welcome back! You have 500 points."

  ### Performance & Offline Targets
  *   **Pass Delivery**: Initial pass bundle generation and edge delivery must occur in < 2.0s.
  *   **Push Latency**: Event-triggered wallet updates (e.g., "Order Ready") must reach APNs/FCM within 500ms of the business owner's action.

  ## Implementation Prompt
  **Objective**: Implement the Autonomous Mobile Pass Engine to automatically issue and update Apple/Google Wallet passes for orders, bookings, and loyalty.

  **User Journey (CUJ) & Acceptance Criteria**:
  1.  **Automatic Provisioning**: When an order is placed or a booking confirmed, the system must automatically generate a `.pkpass` (Apple) and Google Pay object using the tenant's brand colors and logo.
  2.  **Delivery**: The post-checkout success page must render native "Add to Wallet" buttons that successfully install the pass on the user's device.
  3.  **Dynamic Update Sync**: When a tenant updates an order status or booking time via the OHC App, the KAIROS Orchestrator must push an update via APNs/FCM to modify the pass on the customer's lock screen.
  4.  **No Configuration UI**: Do not build a "Pass Designer" tool for the business owner. The design must be completely derived from the global brand tokens managed by the Onboarding Agent.

  **Constraints**:
  Focus on the generation engine and the webhook infrastructure required to receive Apple/Google push token registrations and dispatch updates. Ensure cryptographic isolation between tenant passes. Do not prescribe the specific cryptographic library.

  ## Priority
  `P1`

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
