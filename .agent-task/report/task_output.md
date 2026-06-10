issue_title: "Architecture Design: Offline-Tolerant Tap-to-Pay & In-Person POS"
issue_description: |
  # Architecture Design: Offline-Tolerant Tap-to-Pay & In-Person POS

  ## Problem Statement
  Small business owners with physical presence (like Priya the boutique owner and Fatima the food cart operator) need reliable, mobile-first in-person payment collection. Current solutions force them to either rely on external hardware terminals (e.g., Square hardware, traditional card readers) or use separate Point of Sale (POS) applications that do not natively sync with their online storefront inventory. Additionally, for mobile operations like Fatima's food cart, network connectivity can be slow or intermittent, causing transaction failures, lost sales, and customer frustration.

  OHC currently lacks a unified, offline-tolerant, native Tap-to-Pay system that tightly integrates with the unified inventory and multi-tenant billing engine directly from a single 375px mobile device.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Square:** The industry leader in POS. Offers offline mode natively and integrates Tap-to-Pay on iPhone/Android, but separates online/offline catalogs depending on the tier. Highly hardware-dependent for older devices.
  - **Shopify POS:** Offers robust multi-location inventory and a solid Tap-to-Pay integration. However, the offline mode is restricted in capabilities, and it requires a separate POS app downloaded alongside the standard admin app, fragmenting the owner experience.
  - **Stripe Terminal:** Provides an SDK (Tap to Pay on iPhone & Android) that allows any app to act as a contactless reader without extra hardware. This is a massive enabler for OHC to deliver a zero-hardware POS experience.
  - **OHC Opportunity:** By utilizing the Stripe Terminal Tap to Pay SDK wrapped within our Flutter PWA/App, OHC can turn the owner's existing phone into a fully functional POS terminal. Combined with a local CRDT/SQLite sync engine, we can queue offline transactions and auto-reconcile when connectivity is restored, providing a seamless experience even in low-data environments.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Mobile Client 375px
          A[OHC Mobile App Flutter] --> B(Local Sync Engine SQLite/CRDT)
          A --> C[Stripe Terminal SDK Tap to Pay]
      end
      subgraph Cloud Infrastructure OHC Server
          D[OHC API Gateway REST/gRPC]
          E[Payment & Inventory Queue PostgreSQL SKIP LOCKED]
          F[Central Database Multi-tenant]
      end
      subgraph External Services
          G[Stripe API]
      end

      B -->|Background Sync when Online| D
      C -->|Direct Tokenization| G
      D --> E
      E --> F
      E -->|Finalize & Record Ledger| G
  ```

  ### Mobile UX Flow (375px First)
  - **Catalog View:** Owner selects items from the daily menu/inventory to build a cart. The cart total updates instantly using local state.
  - **Checkout Action:** Tapping "Charge $X.XX" triggers the Tap to Pay overlay natively via the OS.
  - **Tap to Pay Interface:** The screen displays the native system prompt (Apple/Google) to tap a card or device.
  - **Offline State Handling:** If offline, the transaction request is queued locally in the `Local Sync Engine`. The UI immediately shows a "Payment Queued" success state with a warning icon indicating sync pending.
  - **Reconnection:** Upon network restoration, the background worker silently pushes queued transactions, updating the local UI from "Queued" to "Completed."

  ### AI Agent Integration Points
  - **Operations Agent (The Manager):** If an offline transaction depletes inventory to zero upon sync, The Manager agent evaluates the new stock level and drafts an alert to the owner or auto-updates the digital menu board to "Sold Out".
  - **Finance Agent (The Accountant):** Reconciles delayed sync transactions with daily payout reports, explaining anomalies (e.g., "Note: $45 of today's payout was from offline transactions processed late yesterday").

  ### Key Design Decisions
  - **Zero Hardware Requirement:** Exclusively use software-based Tap to Pay via Stripe Terminal to ensure any modern smartphone works out of the box.
  - **Optimistic UI with Local Queue:** Never block the user during a network hiccup. Record the transaction intent locally, allow the owner to serve the next customer, and resolve the payment backend asynchronously.
  - **Unified App:** Do not create a separate OHC POS app. The POS capability must be an integrated mode within the primary OHC command center.

  ## Implementation Prompt
  **User-Facing Outcome:** As an in-person operator (like Priya or Fatima), I can open my OHC app, tap products to add to an order, and tap "Charge." My phone immediately accepts contactless payments. If my internet drops, I can still queue up the transaction, hand over the goods, and trust OHC to process it when my signal returns.

  **CUJ & Acceptance Criteria:**
  1. Set up the foundational models (`Cart`, `OfflineTransactionQueue`, `TerminalSession`) ensuring multi-tenant row-level security.
  2. Implement a local synchronization module in the frontend (Flutter) capable of storing cart/transaction states offline.
  3. Integrate the backend endpoint to receive delayed transaction payloads and process them via Stripe PaymentIntents securely.
  4. Build Playwright E2E tests: A user adds items to a cart, initiates a checkout in a mocked "offline" state, the UI reflects "Queued", the network is mocked to "online", and the backend successfully receives and clears the queue.

  **Priority**: P1
  **Estimated Scope**: Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []