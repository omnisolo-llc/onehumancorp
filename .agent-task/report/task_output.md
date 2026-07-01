issue_title: "Implement Unified Tap-to-Pay and Hardware-Free Point of Sale (POS)"
issue_description: |
  ## Problem Statement
  Small business owners like Priya (boutique owner) and Fatima (food cart operator) need to accept in-person payments seamlessly without purchasing, charging, or pairing external hardware (like physical card readers or dedicated terminals). Currently, OHC lacks a native, hardware-free Point of Sale (POS) solution. This forces owners to use disconnected tools like standalone Square apps, breaking the unified OHC experience and preventing AI agents (Operations, Finance) from acting on real-time sales and inventory data.

  ## Research Report
  Traditional POS systems (Square, early Stripe Terminal, Shopify POS) often require external Bluetooth or USB card readers, adding friction, cost, and points of failure for micro-merchants operating on phones. However, modern APIs like Apple Tap to Pay on iPhone and Stripe's Tap to Pay on Android/iOS allow merchants to use their everyday smartphones as contactless payment terminals. By leveraging these hardware-free SDKs, OHC can provide a zero-friction, integrated POS that directly syncs with our central PostgreSQL ledger, triggering real-time agent workflows. Competitors are adopting this (e.g., Shopify Tap to Pay), making it a critical baseline capability.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as OHC Mobile App (375px)
      participant Native SDK as Stripe Tap to Pay SDK
      participant Stripe as Stripe Backend
      participant OHC Backend as OHC Server
      participant Agent as Finance / Ops Agent

      Owner->>Native SDK: Initiate Payment (Amount / Cart)
      Native SDK->>Customer: Prompt for NFC Tap
      Customer->>Native SDK: Taps Card / Phone
      Native SDK->>Stripe: Process Transaction
      Stripe-->>Native SDK: Success
      Stripe->>OHC Backend: Webhook (payment_intent.succeeded)
      OHC Backend->>Agent: Trigger Post-Sale Workflows (Inventory / Accounting)
      OHC Backend-->>Owner: WebSocket UI Update (Success Screen)
  ```

  ### Mobile UX Flow (375px First)
  1. **POS Home:** A clean, high-contrast screen featuring a large numeric keypad (for quick custom charges) and a drawer for catalog items. Touch targets are generous (min 44x44px).
  2. **Charge Initiation:** The owner taps a prominent "Charge $X.XX" button.
  3. **Tap to Pay Overlay:** The app invokes the native Tap to Pay SDK, showing the OS-level contactless payment UI.
  4. **Success State:** A full-screen, translucent glass success confirmation appears.
  5. **Post-Sale:** A prompt to email/SMS the receipt is shown, while background agents handle inventory deduction and ledger updates.

  ### AI Agent Integration
  - **Finance Assistant:** Intercepts the successful payment webhook to categorize the revenue, update the daily earnings summary, and flag anomalies.
  - **Operations Assistant:** Automatically deductions sold items from the central inventory ledger (coordinating with the Redis Redlock system to prevent sync conflicts with online orders).

  ### Key Design Decisions
  - **Hardware-Free:** Exclusively use Stripe Tap to Pay SDK to eliminate the need for physical card readers, reducing friction for operators like Carlos and Fatima.
  - **Unified Ledger:** Every tap-to-pay transaction is immediately recorded in the multi-tenant PostgreSQL ledger, sharing the exact same data model as online checkout sessions.

  ## Implementation Prompt
  Implement the hardware-free POS and Tap-to-Pay flow for the OHC mobile app.

  **User Facing Outcome:** The owner (e.g., Fatima) can open the OHC app, enter a charge amount or select a pre-order menu item, and tap "Charge." The app should use the device's native NFC capabilities (via Stripe Tap to Pay) to accept a customer's contactless card or phone payment. Upon success, the UI should display a clear confirmation, and the backend must record the transaction in the unified ledger.

  **Acceptance Criteria:**
  1. The POS interface is fully responsive, optimized for a 375px width, utilizing the OHC translucent glass design tokens.
  2. The frontend integrates the relevant Tap to Pay SDK (or a simulated test-mode equivalent if running in a browser/Playwright environment) to process the payment.
  3. The backend receives the payment confirmation, securely updates the PostgreSQL database (ensuring multi-tenant RLS), and triggers the Operations/Finance agents.
  4. End-to-End Playwright tests verify the flow from POS catalog selection to simulated payment success and ledger update. Zero mock data should be used for product states.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
