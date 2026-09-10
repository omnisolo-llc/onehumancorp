issue_title: "POS Terminal Tap-to-Pay & Mobile Inventory Sync Architecture"
issue_description: |
  POS Terminal Tap-to-Pay & Mobile Inventory Sync

  Problem Statement
  Priya (boutique owner) and Carlos (handyman) struggle with accepting payments in person while keeping their online inventory or service calendar synchronized. Currently, when Priya sells a dress in-store, she has to manually update her online inventory. Carlos needs a way to instantly take a deposit on-site without carrying bulky card readers. They need a seamless, mobile-first POS system that supports Tap-to-Pay directly on their smartphones (iPhone/Android) and instantly syncs with their unified OmniSolo inventory and accounting ledger.

  Priority
  P0

  Estimated Scope
  Large

  Research Report
  - Competitive Benchmarking:
    - Shopify POS: Excellent at multi-channel inventory sync, but requires expensive hardware for full functionality and charges high fees for third-party payment gateways.
    - Square: Strong mobile POS and Tap-to-Pay on iPhone, but creates a siloed ecosystem that doesn't easily sync with external appointment booking or manufacturing (MRP) pipelines.
    - Stripe Terminal: Provides native Tap-to-Pay SDKs for iOS and Android, allowing smartphones to act as readers without additional hardware. This is the ideal open infrastructure for OmniSolo.
  - User Pain Points (Operator Communities): Small operators (like Fatima's food cart) complain about the friction of bluetooth pairing with physical card readers during busy lunch rushes. They want to use their existing smartphone.
  - Market Evidence: Tap-to-Pay on iPhone and Android is becoming the industry standard, removing the barrier to entry for mobile merchants.

  Design Doc

  Architecture Diagram
  erDiagram
      TENANT ||--o{ INVENTORY_ITEM : "manages"
      TENANT ||--o{ POS_TRANSACTION : "processes"
      INVENTORY_ITEM ||--o{ POS_TRANSACTION : "included_in"

      POS_TRANSACTION {
          string id PK
          string tenant_id FK
          string status
          bigint amount_cents
          string payment_intent_id
          timestamp completed_at
      }

      INVENTORY_ITEM {
          string id PK
          string tenant_id FK
          string name
          int available_quantity
      }

  Mobile UX Flow (375px First)
  1. Cart/Service Screen: User (Priya/Fatima) taps items or services to add to the cart. Large, high-contrast buttons for outdoor visibility.
  2. Checkout Screen: Prominent "Tap to Pay" button. No technical jargon.
  3. Payment Overlay: Native OS (iOS/Android) Tap-to-Pay interface appears. Customer taps their contactless card or phone.
  4. Success & Sync: Green checkmark success screen. Inventory is instantly deducted, and the transaction is recorded in the OmniSolo ledger.

  AI Agent Integration Points
  - Accounting & Tax Agent: Automatically categorizes the POS transaction and updates cash flow projections.
  - HR & Logistics Agent: If the inventory drops below a threshold, automatically drafts a Purchase Order for the supplier.

  Key Design Decisions
  - Leverage Stripe Terminal SDK: Use the official React Native / Flutter Stripe Terminal SDKs to handle the secure element communication for Tap-to-Pay. This avoids building bespoke payment hardware integrations.
  - Optimistic UI Updates: Inventory counts will update optimistically on the mobile device (SQLite) and sync asynchronously to the cloud PostgreSQL database via the sync engine, ensuring Fatima can keep ringing up customers even if her cell connection drops.

  Implementation Prompt
  Role: Implementer Agent
  Task: Implement the POS Tap-to-Pay and Inventory Sync mobile UI and backend webhook handler.
  CUJ:
  1. A tenant owner opens the mobile app, adds an item to the cart, and selects "Tap to Pay".
  2. The app invokes the Stripe Terminal SDK to process the payment.
  3. Upon success, the app deducts the inventory item locally and sends a sync event to the backend.
  Acceptance Criteria:
  - The UI must be perfectly responsive at 375px width.
  - The Tap-to-Pay button must trigger the native payment flow (or a simulated flow in tests).
  - The inventory count must decrement immediately.
  - The backend must expose an endpoint/webhook to receive the completed transaction and update the Postgres database.
  - No database schemas, API endpoints, or specific function signatures are prescribed here; design them following OmniSolo standards.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
