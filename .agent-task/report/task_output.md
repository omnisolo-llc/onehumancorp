issue_title: "Implement Universal Tap-to-Pay & Offline-Tolerant POS Architecture"
issue_description: |
  # Universal Tap-to-Pay & Offline-Tolerant POS Architecture

  ## Problem Statement
  Operators like Priya (Boutique Owner) and Fatima (Food Cart Operator) run in-person businesses where fast, reliable checkout is critical. Current mobile workflows require switching to external payment apps or bulky hardware. Worse, operators like Fatima often face spotty mobile data (e.g., at food festivals or street corners), causing payment timeouts and lost sales. OHC currently lacks a built-in, offline-tolerant Point of Sale (POS) flow with native Tap-to-Pay (via Stripe Terminal SDK) that functions seamlessly on a 375px mobile viewport and degrades gracefully during network interruptions.

  ## Research Report
  - **Market Context**: Square, Shopify POS, and Wix all offer native mobile POS experiences. However, their tap-to-pay experiences are often walled off from their core agentic assistants. Shopify POS operates robustly offline but requires syncing sessions upon reconnection.
  - **Stripe Terminal SDK**: Supports "Tap to Pay on iPhone" and "Tap to Pay on Android", turning the device itself into a contactless reader without extra hardware.
  - **The Gap in OHC**: To truly replace a bundle of disjointed apps, OHC must allow a business owner to accept a card scan natively within the assistant feed while seamlessly logging the transaction to the unified tenant ledger.
  - **Offline Resilience**: Offline queueing (CRDTs or local SQLite sync) is required to ensure items can be added to the cart and the transaction can be staged even if the connection drops momentarily before payment processing.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Operator (375px UI)
      participant OHCApp as OHC Flutter App (Local SQLite)
      participant StripeSDK as Stripe Terminal SDK
      participant OHCBackend as OHC API (Go)
      participant AgentQueue as AI Job Queue (PostgreSQL)

      Owner->>OHCApp: Taps "Charge $15.00 (Tap to Pay)"
      OHCApp->>StripeSDK: Initialize Transaction
      StripeSDK-->>Owner: Present Card/Device
      StripeSDK->>OHCApp: Payment Intent Authorized
      OHCApp->>OHCBackend: Sync Transaction (Idempotency Key)
      OHCBackend->>AgentQueue: Enqueue "Post-Sale Actions" (Receipt, Inventory)
      AgentQueue-->>OHCBackend: Agent drafts follow-up
      OHCBackend-->>OHCApp: Transaction Complete
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  1. **Quick Charge / POS Screen**: A large numeric keypad (touch targets > 44px) and an "Add Item" button overlay. High contrast, usable outdoors.
  2. **Payment Method Sheet**: A bottom sheet (translucent glass styling) slides up with options: "Tap to Pay", "Cash", "Send Link".
  3. **Tap to Pay Activation**: The screen transitions to the native OS contactless payment interface.
  4. **Success Card & Agent Feed**: Upon success, a large green success token appears. Simultaneously, an "Action Card" is drafted in the Agent Feed: *"Send digital receipt to new customer?"*

  ### AI Agent Integration Points
  - **Operations Assistant**: Automatically decrements inventory and checks for low-stock thresholds immediately after the transaction syncs.
  - **Customer Assistant**: If the customer provides an email/phone for the receipt, the agent cross-references the CRM to merge profiles and drafts an optional "Thank You / Request Review" follow-up message.
  - **Finance Assistant**: Logs the offline/online transaction to the daily ledger summary.

  ### Key Design Decisions and Why
  - **Local-First SQLite Sync**: Transactions are staged in a local SQLite table before syncing to the backend. This allows the operator to build the cart and review the order even with zero signal.
  - **Stripe Terminal for Tap-to-Pay**: Eliminates the need for external hardware, making OHC instantly usable for operators like Carlos or Fatima with only their smartphones.
  - **Idempotent Webhooks**: All payment syncs use UUID v4 keys to ensure that a reconnecting device doesn't double-charge the tenant's customer.

  ## Implementation Prompt
  **User-Facing Outcome**: When the user opens the OHC mobile app, they can access a "New Sale" button from the main feed. They can enter an amount, tap "Tap to Pay", and accept a contactless payment. If the network is spotty, the app must queue the transaction and sync it once the connection is restored, while instantly reflecting the sale in the local daily summary.

  **CUJ / Acceptance Criteria**:
  1. User logs into OHC on a 375px mobile viewport.
  2. User taps "New Sale" and enters $15.00.
  3. User selects "Tap to Pay" (using a mocked Stripe Terminal adapter for E2E tests).
  4. The UI displays a success state and returns to the Agent Feed.
  5. The Agent Feed displays a drafted card: "Transaction successful. Send receipt?"
  6. The test must simulate network offline/online transitions, proving the transaction is preserved locally and synced idempotently when online.
  7. Implement the backend Go handlers, Flutter UI components (macOS translucent glass styled), and the SQLite synchronization logic. ZERO mock data in the production UI state.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
