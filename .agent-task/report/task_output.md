issue_title: "[research] Architect High-Performance, Mobile-First Stripe Terminal POS SDK Integration for Tap-to-Pay"
issue_description: |
  # Research Report: High-Performance, Mobile-First Stripe Terminal POS Integration

  ## Problem Statement
  Small business owners with physical presences (e.g., Priya the boutique owner, Fatima the food cart operator) need to accept in-person payments seamlessly. Currently, they often resort to using disparate systems (a separate POS system alongside their online store) which fragments inventory, customer data, and financial reporting. They need a unified "tap-to-pay" solution directly integrated into their OHC mobile app, functioning even on low-end devices or slow networks, to accept payments effortlessly without extra hardware or complex setup.

  ## Research & Competitive Analysis
  - **Shopify POS:** Offers a robust, unified system but is often overly complex for a single-person business and requires proprietary hardware for some features.
  - **Square:** The dominant player in simple POS, but acts as a walled garden, separate from an online-first presence.
  - **Stripe Terminal (Tap to Pay on iPhone/Android):** Provides an SDK that allows standard smartphones to act as contactless card readers without additional hardware. This aligns perfectly with OHC's mobile-first, no-extra-hardware ethos.

  **Key Insight:** By leveraging Stripe Terminal's Tap to Pay SDK directly within the OHC Flutter app, we can instantly transform a user's phone into a POS terminal. This unifies in-person and online sales data in real-time, feeding directly into the "Finance & Payments" and "Operations" AI departments.

  ## Proposed Architecture Design
  ### 1. Mobile Client (Flutter)
  - Integrate the official `stripe_terminal` Flutter SDK.
  - Implement a highly-optimized, responsive UI specifically designed for 375px viewports. The flow must be: Enter Amount -> Tap 'Charge' -> Native OS Tap-to-Pay overlay appears.
  - **Offline/Low-Connectivity Resilience:** Implement a queue for transaction records. While Stripe requires connectivity for authorization, the app must handle network drops gracefully during the flow, offering clear, non-technical feedback and auto-retrying token fetching.

  ### 2. Backend API (Go/gRPC)
  - `PaymentService` exposes endpoints to generate Stripe ConnectionTokens (required by the SDK).
  - Webhook listener to handle asynchronous payment intent status updates (e.g., `payment_intent.succeeded`).
  - Strict multi-tenant isolation: Ensure the Stripe account ID used corresponds to the authenticated tenant.

  ### 3. AI Coordination
  - **Finance & Payments (The Accountant):** Automatically logs the successful transaction, updates daily revenue metrics, and flags it as "In-Person."
  - **Operations (The Manager):** If the payment is linked to specific inventory items, deducts them immediately.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User as Mobile App (Flutter)
      participant Stripe as Stripe Terminal API
      participant Backend as OHC Backend (Go)
      participant AI as AI Agents
      participant DB as PostgreSQL Database

      User->>Backend: Request ConnectionToken
      Backend->>Stripe: Create ConnectionToken
      Stripe-->>Backend: Token response
      Backend-->>User: ConnectionToken string

      User->>User: Collect amount ($45) via Keypad
      User->>Backend: Create PaymentIntent (Amount: 4500, currency: usd)
      Backend->>Stripe: Create PaymentIntent
      Stripe-->>Backend: Client Secret
      Backend-->>User: Client Secret

      User->>Stripe: Process Payment via Native OS Overlay
      Stripe-->>User: Process Status (Success/Fail)

      User->>Backend: Notify Payment Attempt Completed
      Stripe-->>Backend: Async Webhook (payment_intent.succeeded)
      Backend->>DB: Record Payment (Tenant Isolated)
      Backend->>AI: Trigger "Accountant" & "Manager" flows
  ```

  ### Visual & UX Guidelines (Translucent Glass)
  - The POS view should feature a clean, large numeric keypad.
  - Use UniFi-style modular cards for item summaries.
  - The 'Charge' button should have a prominent, satisfying micro-animation and utilize the signature 20px blur backdrop.

  ## Implementation Prompt
  Implement the Stripe Terminal Tap-to-Pay integration.
  **CUJ:** As Priya (boutique owner), I open the OHC app on my iPhone, go to the "In-Person Sale" tab, enter $45.00, and tap "Charge." The native Apple Tap-to-Pay interface appears. The customer taps their card. The app shows a success checkmark and the $45 is added to today's dashboard.
  **Acceptance Criteria:**
  1. Backend API provides a secure endpoint for ConnectionTokens.
  2. Flutter app integrates the Stripe Terminal SDK and handles the connection flow.
  3. UI is fully responsive at 375px and adheres to the Glassmorphism design system.
  4. Successful payments are recorded in the PostgreSQL database with correct tenant isolation.
  5. 100% unit test coverage for new backend logic and a Playwright E2E test verifying the UI flow (using mocked Stripe SDK responses *only* at the device edge, real backend flow).

  **Estimated Scope:** Medium

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
