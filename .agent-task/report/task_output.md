issue_title: "Hardware-free Tap-to-Pay & Offline-Tolerant mPOS Integration"
issue_description: |
  ## Problem Statement
  Owners like Priya (in-store boutique) and Fatima (food cart) need to collect in-person payments seamlessly without requiring expensive secondary hardware like Square terminals. Current solutions either require bulky hardware, complicated Bluetooth pairings, or switching apps away from their core operations and customer inboxes. They need a unified solution within the OHC app that supports hardware-free Tap-to-Pay (NFC) while being resilient to slow or offline network conditions, ensuring they never lose a sale even at a busy, poorly-connected food cart or pop-up shop.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Square:** The industry standard for mobile Point of Sale (mPOS). Tap-to-Pay on iPhone/Android is seamless, and they support a robust offline mode (stores encrypted card data locally and processes when reconnected).
  - **Shopify POS:** Excellent hardware-free tap-to-pay on mobile but involves a very complex initial setup and separates POS from the core e-commerce admin app. Highly reliant on good network connectivity.
  - **Stripe Terminal SDK:** Supports Tap-to-Pay on compatible iOS and Android devices without external readers, providing the technical foundation for native app integration.
  - **OHC Opportunity:** By integrating Stripe Terminal SDK's Tap-to-Pay capabilities directly into the OHC Flutter app, we can provide immediate in-person payment collection. The key differentiator is routing these transactions through OHC's offline-tolerant queue and synchronizing them automatically with our unified inventory and customer graph, all within the single Owner Assistant app.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[OHC Flutter App 375px] -->|NFC Tap-to-Pay| B(Stripe Terminal SDK)
      A -->|Network Fails| C[Local Encrypted Queue SQLite]
      A -->|Network Success| D[OHC Core API Go]
      C -->|Background Sync| D
      D -->|Process Payment Intent| E[Stripe API]
      D -->|Record Transaction| F[Unified OHC DB PostgreSQL]
      F -->|Trigger Event| G[AI Finance Assistant]
      G -->|Daily Summary| H[Owner Feed]
  ```

  ### Mobile UX Flow
  1. Owner opens OHC App on their phone (375px viewport), taps "New Sale" or opens an existing pending order (e.g., Fatima's pre-orders).
  2. App displays a clean, Apple-like translucent glass cart summary and a large, primary "Tap to Pay" action button (min 44x44px).
  3. Owner taps the button; the native iOS/Android NFC Tap-to-Pay sheet takes over the screen.
  4. Customer taps their physical card or phone to the owner's device.
  5. If online, the transaction completes instantly, returning to the app with a vibrant green success token.
  6. If offline/flaky network, the transaction is securely queued locally. The app shows a "Payment Captured (Offline Sync Pending)" yellow status token, giving the owner confidence to hand over the goods and move to the next customer.

  ### AI Agent Integration
  - **Finance Assistant:** Reconciles offline payments once synchronized and includes them in the daily plain-language performance summary. Proactively flags anomalies or rejected offline payments for the owner's attention in the feed.
  - **Operations Assistant:** Automatically adjusts inventory levels in the central catalog based on the in-person sale. If an item sells out via Tap-to-Pay, it instantly triggers a sync to pause online availability to prevent double-selling (crucial for Priya's limited inventory).

  ## Implementation Prompt
  Implement the offline-tolerant Tap-to-Pay capability for the OHC platform.
  1. Integrate the Stripe Terminal SDK wrapper in the Flutter frontend for native iOS/Android Tap-to-Pay.
  2. Build a local SQLite-backed queue in Flutter to capture and encrypt payment intents when the network is unreachable.
  3. Create a background sync worker that robustly flushes the offline queue to the Go API when connectivity is restored.
  4. Implement the backend Go endpoints to process these synced Payment Intents idempotently against the Stripe API and record them in the unified PostgreSQL database.
  5. Ensure all UI elements adhere to the OHC Premium Token library (translucent materials, minimum 44x44px touch targets, clear online/offline status indicators).
  6. Add comprehensive E2E Playwright tests covering the offline queueing and background synchronization CUJ.
  Estimated Scope: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
